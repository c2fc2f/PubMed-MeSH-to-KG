//! Management and iteration of PubMed data chunks.
//!
//! This module provides the [`Chunks`] struct, which implements an
//! iterator-like pattern to fetch, decompress, and parse XML data from the
//! NCBI baseline. It ensures that large datasets can be processed
//! sequentially without loading the entire archive into memory.

pub mod models;

use std::{
    collections::VecDeque, fs::File, io::BufReader, path::PathBuf, pin::Pin,
};

use crate::{PubMed, error::PubMedError};
use async_compression::tokio::{bufread, write};
use futures::StreamExt;
use models::PubmedArticleSet;
use reqwest::{Client, Response, Url};
use tokio::io::AsyncWriteExt;
use tokio_util::io::{StreamReader, SyncIoBridge};

/// An iterable collection of parsed PubMed XML records.
///
/// This struct holds the metadata for available XML chunks and provides
/// methods to iterate over the actual data.
pub struct Chunks {
    /// The internal HTTP client used for making requests to the NCBI FTP.
    client: reqwest::Client,

    /// The resolved base URL (e.g., the default NCBI URL or a custom mirror).
    base_url: Url,

    /// The directory where downloaded `pubmedXXnXXXX.xml` files will be
    /// cached. If [`None`], caching is disabled and files are kept in memory.
    cache: Option<PathBuf>,

    /// The list of discovered filenames to be processed.
    files: VecDeque<String>,
}

impl Chunks {
    /// Creates a new [`Chunks`] instance.
    pub(crate) fn new(pubmed: &PubMed, files: VecDeque<String>) -> Self {
        Self {
            client: pubmed.client.clone(),
            base_url: pubmed.base_url.clone(),
            cache: pubmed.cache.clone(),
            files,
        }
    }
}

/// A single parsed unit of PubMed data.
///
/// This represents a complete `<PubmedArticleSet>` parsed from a single
/// compressed XML file. It typically contains a collection of multiple
/// [`PubmedArticle`] and [`PubmedBookArticle`] entries.
type Chunk = PubmedArticleSet;

impl Iterator for Chunks {
    type Item =
        Pin<Box<dyn Future<Output = Result<Chunk, PubMedError>> + Send>>;

    fn next(&mut self) -> Option<Self::Item> {
        let file: String = self.files.pop_front()?;
        let url: Url =
            self.base_url.join(&file).expect("Invalid URL construction");
        let client: Client = self.client.clone();

        let cache_dir: Option<PathBuf> = self.cache.clone();

        Some(Box::pin(async move {
            if let Some(dir) = &cache_dir {
                let mut file_path: PathBuf = dir.join(&file);
                file_path.set_extension("");

                if !file_path.exists() {
                    let tmp_path: PathBuf =
                        file_path.with_added_extension("tmp");

                    tokio::fs::create_dir_all(&dir)
                        .await
                        .map_err(PubMedError::Cache)?;

                    let response: Response = client
                        .get(url)
                        .send()
                        .await
                        .and_then(|e| e.error_for_status())
                        .map_err(PubMedError::Request)?;

                    let dest_file: tokio::fs::File =
                        tokio::fs::File::create(&tmp_path)
                            .await
                            .map_err(PubMedError::Cache)?;

                    let mut stream = response.bytes_stream();

                    let mut decoder: write::GzipDecoder<tokio::fs::File> =
                        write::GzipDecoder::new(dest_file);
                    while let Some(input) = stream.next().await {
                        decoder
                            .write_all(
                                input.map_err(PubMedError::Request)?.as_ref(),
                            )
                            .await
                            .map_err(PubMedError::Cache)?;
                    }
                    decoder.shutdown().await.map_err(PubMedError::Cache)?;

                    tokio::fs::rename(&tmp_path, &file_path)
                        .await
                        .map_err(PubMedError::Cache)?;
                }

                tokio::task::spawn_blocking(move || {
                    let file: File =
                        File::open(file_path).map_err(PubMedError::Cache)?;
                    let buf_reader: BufReader<File> =
                        std::io::BufReader::new(file);

                    #[cfg(feature = "debug_path")]
                    {
                        let mut de = quick_xml::de::Deserializer::from_reader(
                            buf_reader,
                        );
                        serde_path_to_error::deserialize(&mut de)
                            .map_err(PubMedError::Parsing)
                    }

                    #[cfg(not(feature = "debug_path"))]
                    {
                        quick_xml::de::from_reader(buf_reader)
                            .map_err(PubMedError::Parsing)
                    }
                })
                .await
                .expect("Tokio blocking task panicked")
            } else {
                let response: Response = client
                    .get(url)
                    .send()
                    .await
                    .and_then(|e| e.error_for_status())
                    .map_err(PubMedError::Request)?;

                let stream = response.bytes_stream().map(|res| {
                    res.map_err(|e| {
                        std::io::Error::other(PubMedError::Request(e))
                    })
                });

                let stream_reader: StreamReader<_, _> =
                    StreamReader::new(stream);

                let decoder: bufread::GzipDecoder<StreamReader<_, _>> =
                    bufread::GzipDecoder::new(stream_reader);

                let sync_reader: SyncIoBridge<_> = SyncIoBridge::new(decoder);

                let buf_reader: BufReader<_> = BufReader::new(sync_reader);

                tokio::task::spawn_blocking(move || {
                    #[cfg(feature = "debug_path")]
                    {
                        let mut de = quick_xml::de::Deserializer::from_reader(
                            buf_reader,
                        );
                        serde_path_to_error::deserialize(&mut de)
                            .map_err(PubMedError::Parsing)
                    }

                    #[cfg(not(feature = "debug_path"))]
                    {
                        quick_xml::de::from_reader(buf_reader)
                            .map_err(PubMedError::Parsing)
                    }
                })
                .await
                .expect("Tokio blocking task panicked")
            }
        }))
    }
}
