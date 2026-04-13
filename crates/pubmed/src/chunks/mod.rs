//! Management and iteration of PubMed data chunks.
//!
//! This module provides the [`Chunks`] struct, which implements an
//! iterator-like pattern to fetch, decompress, and parse XML data from the
//! NCBI baseline. It ensures that large datasets can be processed
//! sequentially without loading the entire archive into memory.

pub mod models;

use std::{
    collections::VecDeque, io::BufReader, path::PathBuf, pin::Pin, sync::Arc,
};

use crate::{PubMed, error::PubMedError};
use async_compression::tokio::{bufread, write};
use futures::StreamExt;
use models::PubmedArticle;
use models::PubmedArticleSet;
use models::processor::PubmedArticleSetSeed;
use reqwest::{Client, Response, Url};
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tokio_util::io::{StreamReader, SyncIoBridge};

/// A wrapper for a callback function used during streaming deserialization.
///
/// This type is used as a generic parameter in [`Chunks`] to inject custom
/// logic that processes each [`PubmedArticle`] as it is parsed,
/// preventing large memory allocations.
#[derive(Default, Clone)]
pub struct Processor<F>(Arc<F>)
where
    F: Fn(PubmedArticle) + Send + Sync;

/// A marker type indicating that no processor has been assigned to the
/// [`Chunks`] instance.
///
/// When [`Chunks`] holds this type, it behaves as a file manager or metadata
/// holder without active data processing capabilities.
#[derive(Debug, Default, Clone, Copy)]
pub struct NoProcessor;

/// An iterable collection of parsed PubMed XML records.
///
/// This struct holds the metadata for available XML chunks and provides
/// methods to iterate over the actual data.
pub struct Chunks<T = NoProcessor> {
    /// The internal HTTP client used for making requests to the NCBI FTP.
    client: reqwest::Client,

    /// The resolved base URL (e.g., the default NCBI URL or a custom mirror).
    base_url: Url,

    /// The directory where downloaded `pubmedXXnXXXX.xml` files will be
    /// cached. If [`None`], caching is disabled and files are kept in memory.
    cache: Option<PathBuf>,

    /// The list of discovered filenames to be processed.
    files: VecDeque<String>,

    /// The processing logic assigned to this instance.
    ///
    /// This will either be [`NoProcessor`] or a [`Processor`] containing
    /// a user-defined closure.
    processor: T,
}

impl Chunks {
    /// Creates a new [`Chunks`] instance.
    pub(crate) fn new(pubmed: &PubMed, files: VecDeque<String>) -> Self {
        Self {
            client: pubmed.client.clone(),
            base_url: pubmed.base_url.clone(),
            cache: pubmed.cache.clone(),
            files,
            processor: NoProcessor,
        }
    }

    /// Attaches a processing callback to the chunk manager.
    ///
    /// This method consumes the current [`Chunks<NoProcessor>`] and returns a
    /// [`Chunks<Processor<F>>`]. This enables streaming deserialization,
    /// where each article is passed to the provided closure as soon as it is
    /// parsed from the XML stream.
    ///
    /// # Arguments
    /// * `processor` - A closure or function that accepts a
    ///   [`PubmedArticle`]. This avoids collecting the entire
    ///   [`PubmedArticleSet`] into memory.
    ///
    /// # Performance
    /// Using a processor avoids loading the entire [`PubmedArticleSet`] into
    /// memory, reducing RAM overhead.
    ///
    /// # Example
    /// ```rust
    /// let active_chunks = chunks.processor(|article: PubmedArticle| {
    ///     println!("Processing: {}", article.medline_citation.pmid.value);
    /// });
    /// ```
    pub fn processor<F>(self, processor: F) -> Chunks<Processor<F>>
    where
        F: Fn(PubmedArticle) + Send + Sync,
    {
        Chunks {
            client: self.client,
            base_url: self.base_url,
            cache: self.cache,
            files: self.files,
            processor: Processor(Arc::new(processor)),
        }
    }
}

/// Runs the [`quick-xml`] deserializer on `reader`.
///
/// * `processor = Some(f)` — streaming mode: each article is forwarded to `f`
///   as soon as it is parsed. No [`Vec`] is ever built. Returns `Ok(None)`.
/// * `processor = None` — bulk mode: the entire set is collected into a
///   [`PubmedArticleSet`]. Returns `Ok(Some(set))`.
///
/// Both modes transparently support the `debug_path` feature flag.
///
/// # Errors
///
/// Returns [`PubMedError::Parsing`] if `quick-xml` fails to deserialize the
/// document.
fn deserialize_chunk<R, F>(
    reader: BufReader<R>,
    processor: Option<&F>,
) -> Result<Option<PubmedArticleSet>, PubMedError>
where
    R: std::io::Read,
    F: Fn(PubmedArticle),
{
    use serde::de::DeserializeSeed as _;

    if let Some(f) = processor {
        let seed: PubmedArticleSetSeed<'_, F> =
            PubmedArticleSetSeed { processor: f };

        #[cfg(feature = "debug_path")]
        {
            let mut de = quick_xml::de::Deserializer::from_reader(reader);
            let mut track = serde_path_to_error::Track::new();
            let tracked =
                serde_path_to_error::Deserializer::new(&mut de, &mut track);
            seed.deserialize(tracked).map(|_: ()| None).map_err(|e| {
                // Attach the tracked path to the raw DeError
                let path = track.path().clone();
                PubMedError::Parsing(serde_path_to_error::Error::new(path, e))
            })
        }

        #[cfg(not(feature = "debug_path"))]
        {
            let mut de: quick_xml::de::Deserializer<_> =
                quick_xml::de::Deserializer::from_reader(reader);
            seed.deserialize(&mut de)
                .map(|_: ()| None)
                .map_err(PubMedError::Parsing)
        }
    } else {
        #[cfg(feature = "debug_path")]
        {
            let mut de = quick_xml::de::Deserializer::from_reader(reader);
            serde_path_to_error::deserialize(&mut de)
                .map(Some)
                .map_err(PubMedError::Parsing)
        }

        #[cfg(not(feature = "debug_path"))]
        {
            quick_xml::de::from_reader(reader)
                .map(Some)
                .map_err(PubMedError::Parsing)
        }
    }
}

/// Core async logic shared by both iterator impls.
///
/// Downloads (and optionally caches to disk) one `.xml.gz` file, then runs
/// [`deserialize_chunk`] inside a `spawn_blocking` task.
///
/// * When `processor` is `Some`, articles are streamed to the callback and
///   `Ok(None)` is returned.
/// * When `processor` is `None`, the full [`PubmedArticleSet`] is returned
///   as `Ok(Some(set))`.
///
/// # Errors
///
/// Returns [`PubMedError::Cache`] on filesystem I/O failures and
/// [`PubMedError::Request`] on network failures.
async fn process_chunk<F>(
    client: Client,
    base_url: Url,
    cache: Option<PathBuf>,
    file: String,
    processor: Option<Arc<F>>,
) -> Result<Option<PubmedArticleSet>, PubMedError>
where
    F: Fn(PubmedArticle) + Send + Sync + 'static,
{
    let url: Url = base_url.join(&file).expect("Invalid URL construction");

    if let Some(dir) = &cache {
        let mut file_path: PathBuf = dir.join(&file);
        file_path.set_extension("");

        if !file_path.exists() {
            let tmp_path: PathBuf = file_path.with_added_extension("tmp");

            tokio::fs::create_dir_all(dir)
                .await
                .map_err(PubMedError::Cache)?;

            let response: Response = client
                .get(url)
                .send()
                .await
                .and_then(|r| r.error_for_status())
                .map_err(PubMedError::Request)?;

            let dest: File =
                File::create(&tmp_path).await.map_err(PubMedError::Cache)?;

            let mut decoder: write::GzipDecoder<File> =
                write::GzipDecoder::new(dest);
            let mut stream = response.bytes_stream();

            while let Some(chunk) = stream.next().await {
                decoder
                    .write_all(chunk.map_err(PubMedError::Request)?.as_ref())
                    .await
                    .map_err(PubMedError::Cache)?;
            }
            decoder.shutdown().await.map_err(PubMedError::Cache)?;

            tokio::fs::rename(&tmp_path, &file_path)
                .await
                .map_err(PubMedError::Cache)?;
        }

        tokio::task::spawn_blocking(move || {
            let file: std::fs::File =
                std::fs::File::open(file_path).map_err(PubMedError::Cache)?;
            let buf = BufReader::new(file);
            deserialize_chunk(buf, processor.as_deref())
        })
        .await
        .expect("Tokio blocking task panicked")
    } else {
        let response: Response = client
            .get(url)
            .send()
            .await
            .and_then(|r| r.error_for_status())
            .map_err(PubMedError::Request)?;

        let byte_stream = response.bytes_stream().map(|res| {
            res.map_err(|e| std::io::Error::other(PubMedError::Request(e)))
        });

        let decoder: bufread::GzipDecoder<_> =
            bufread::GzipDecoder::new(StreamReader::new(byte_stream));
        let buf: BufReader<_> = BufReader::new(SyncIoBridge::new(decoder));

        tokio::task::spawn_blocking(move || {
            deserialize_chunk(buf, processor.as_deref())
        })
        .await
        .expect("Tokio blocking task panicked")
    }
}

impl Iterator for Chunks<NoProcessor> {
    type Item = Pin<
        Box<dyn Future<Output = Result<PubmedArticleSet, PubMedError>> + Send>,
    >;

    fn next(&mut self) -> Option<Self::Item> {
        let file: String = self.files.pop_front()?;
        let client: Client = self.client.clone();
        let base_url: Url = self.base_url.clone();
        let cache: Option<PathBuf> = self.cache.clone();

        Some(Box::pin(async move {
            process_chunk::<fn(PubmedArticle)>(
                client, base_url, cache, file, None,
            )
            .await
            .map(|opt| opt.expect("bulk deserialization returned None"))
        }))
    }
}

impl<F> Iterator for Chunks<Processor<F>>
where
    F: Fn(PubmedArticle) + Send + Sync + 'static,
{
    type Item = Pin<Box<dyn Future<Output = Result<(), PubMedError>> + Send>>;

    fn next(&mut self) -> Option<Self::Item> {
        let file: String = self.files.pop_front()?;
        let client: Client = self.client.clone();
        let base_url: Url = self.base_url.clone();
        let cache: Option<PathBuf> = self.cache.clone();
        let processor: Arc<F> = Arc::clone(&self.processor.0);

        Some(Box::pin(async move {
            process_chunk(client, base_url, cache, file, Some(processor))
                .await
                .map(|_| ())
        }))
    }
}

