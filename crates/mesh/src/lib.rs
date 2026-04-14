//! # mesh
//!
//! A streaming Rust client for the [NLM MeSH XML dataset].
//!
//! MeSH (Medical Subject Headings) is the NLM's controlled vocabulary for
//! indexing biomedical literature. NLM publishes the full dataset as a set of
//! large XML files. This crate fetches those files over HTTP, parses the
//! with [`quick-xml`] and [`serde`], and forwards each record to a
//! caller-supplied callback — without ever allocating the entire dataset in
//! memory.
//!
//! [NLM MeSH XML dataset]: https://www.nlm.nih.gov/mesh/meshhome.html
//!
//! ## Dataset overview
//!
//! The MeSH dataset is split into three record types, each stored in a
//! dedicated XML file on the NLM server:
//!
//! | Method | File pattern | Record type |
//! |---|---|---|
//! | [`MeSH::descriptor`] | `descXXXX.xml` | [`DescriptorRecord`] |
//! | [`MeSH::qualifier`] | `qualXXXX.xml` | [`QualifierRecord`] |
//! | [`MeSH::supplemental`] | `suppXXXX.xml` | [`SupplementalRecord`] |
//!
//! ## Quick start
//!
//! ```rust,no_run
//! use mesh::MeSH;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = MeSH::new(); // uses default NLM URL + system cache dir
//!
//!     client.descriptor(|record| {
//!         println!("{} — {}", record.ui, record.name.value);
//!     }).await?;
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Configuration
//!
//! Use [`MeSH::builder`] to customise the HTTP client, the base URL, or the
//! cache directory:
//!
//! ```rust,no_run
//! use mesh::MeSH;
//! use std::path::PathBuf;
//!
//! let client = MeSH::builder()
//!     .cache(Some(PathBuf::from("/tmp/mesh-cache")))
//!     .build();
//! ```
//!
//! Set the cache to [`None`] to disable on-disk caching and stream XML
//! directly into the parser:
//!
//! ```rust,no_run
//! let client = mesh::MeSH::builder()
//!     .cache(None)
//!     .build();
//! ```
//!
//! ## Caching
//!
//! When a cache directory is configured (the default when using
//! [`MeSH::default`]), the downloaded XML files are written to disk on the
//! first call and reused on subsequent calls. Files are written atomically via
//! a `.tmp` rename to avoid leaving partially-written data.
//!
//! ## Streaming design
//!
//! Each XML file can be hundreds of megabytes. The parsing pipeline avoids
//! collecting records into a [`Vec`]:
//!
//! 1. The HTTP response body is handed to a [`BufReader`] inside a
//!    `spawn_blocking` task (or read from cache).
//! 2. A custom [`serde::de::DeserializeSeed`] implementation walks the XML
//!    stream element-by-element via [`quick-xml`]'s streaming deserializer.
//! 3. Each fully-parsed record is forwarded immediately to the callback.
//!
//! The three [`streaming_set_seed!`], [`streaming_processor!`], and
//! [`streaming_fetch_method!`] macros encapsulate this pipeline and are
//! re-exported so that downstream crates can reuse the same pattern with
//! custom record types.
//!
//! ## Feature flags
//!
//! | Feature | Description |
//! |---|---|
//! | `debug_path` | Wraps XML parse errors with [`serde_path_to_error`], adding the JSON/XML path to error messages. Useful during development; adds a dependency. |
//!
//! [`BufReader`]: std::io::BufReader
//! [`DescriptorRecord`]: crate::descriptor::models::DescriptorRecord
//! [`QualifierRecord`]: crate::qualifier::models::QualifierRecord
//! [`SupplementalRecord`]: crate::supplemental::models::SupplementalRecord

pub mod descriptor;
pub mod error;
pub mod qualifier;
mod streaming;
pub mod supplemental;

use std::path::PathBuf;

use reqwest::Url;

use crate::{
    descriptor::{models::DescriptorRecord, process_descriptor},
    error::MeSHError,
    qualifier::{models::QualifierRecord, process_qualifier},
    supplemental::{models::SupplementalRecord, process_supplemental},
};

/// The default NLM FTP URL
const DEFAULT_URL: &str =
    "https://nlmpubs.nlm.nih.gov/projects/mesh/MESH_FILES/xmlmesh/";

/// A client for handling operations on the MeSH dataset.
///
/// This struct manages the connection to the NLM servers and stores
/// the configuration required for dataset retrieval.
pub struct MeSH {
    /// The internal HTTP client used for making requests to the NLM FTP.
    client: reqwest::Client,

    /// The resolved base URL (e.g., the default MeSH URL or a custom mirror).
    base_url: Url,

    /// The directory where downloaded files (e.g., `descXXXX.xml`) will be
    /// cached. If [`None`], caching is disabled and files are kept in memory.
    ///
    /// # Default Behavior
    ///
    /// When using [`MeSH::default()`], this automatically resolves to a
    /// `mesh-ftp`subdirectory within the system's standard cache location
    /// (determined via [`dirs::cache_dir`]).
    cache: Option<PathBuf>,
}

/// A [`MeSHBuilder`] can be used to create a [`MeSH`] with custom
/// configuration.
#[derive(Default)]
pub struct MeSHBuilder {
    /// The HTTP client used for making requests.
    /// If [`None`], a default [`reqwest::Client`] is created during
    /// [`MeSHBuilder::build()`].
    client: Option<reqwest::Client>,

    /// The base URL for the MeSH dataset.
    /// If [`None`], the default NLM FTP URL is used during
    /// [`MeSHBuilder::build()`].
    base_url: Option<Url>,

    /// The directory where downloaded files (e.g., `descXXXX.xml`) will be
    /// cached. If [`None`], caching is disabled and files are kept in memory.
    cache: Option<PathBuf>,
}

impl MeSHBuilder {
    /// Creates a [`MeSHBuilder`] to configure a [`MeSH`].
    pub fn new() -> Self {
        Default::default()
    }

    /// Sets the HTTP client for the [`MeSH`] instance.
    ///
    /// This allows you to provide a pre-configured [`reqwest::Client`] with
    /// custom headers, timeouts, or proxy settings.
    pub fn client(self, client: reqwest::Client) -> Self {
        Self {
            client: Some(client),
            base_url: self.base_url,
            cache: None,
        }
    }

    /// Sets the base URL for the MeSH API.
    ///
    /// If not provided, the default NLM FTP URL will be used during the
    /// build process.
    pub fn base_url(self, url: Url) -> Self {
        Self {
            client: self.client,
            base_url: Some(url),
            cache: self.cache,
        }
    }

    /// Sets the cache directory for storing downloaded files (e.g.,
    /// `descXXXX.xml`).
    ///
    /// When a cache directory is set, downloaded baseline files are saved
    /// here to avoid redundant network requests in future operations. If set
    /// to [`None`], caching is disabled.
    pub fn cache(self, cache: Option<PathBuf>) -> Self {
        Self {
            client: self.client,
            base_url: self.base_url,
            cache,
        }
    }

    /// Returns a [`MeSH`] that uses this [`MeSHBuilder`] configuration.
    ///
    /// # Panics
    ///
    /// Panics if no base URL provided and the default URL is not valid.
    pub fn build(self) -> MeSH {
        MeSH {
            client: self.client.unwrap_or_default(),
            base_url: self.base_url.unwrap_or_else(|| {
                Url::parse(DEFAULT_URL)
                    .expect("Hardcoded DEFAULT_URL is invalid")
            }),
            cache: self.cache,
        }
    }
}

impl Default for MeSH {
    fn default() -> Self {
        Self {
            client: Default::default(),
            base_url: Url::parse(DEFAULT_URL)
                .expect("Hardcoded DEFAULT_URL is invalid"),
            cache: dirs::cache_dir().map(|p| p.join("mesh-ftp")),
        }
    }
}

impl MeSH {
    /// Creates a [`MeSHBuilder`] to configure a [`MeSH`].
    ///
    /// # Panics
    ///
    /// Panics if the default URL is not valid.
    pub fn builder() -> MeSHBuilder {
        MeSHBuilder {
            client: Default::default(),
            base_url: Some(
                Url::parse(DEFAULT_URL)
                    .expect("Hardcoded DEFAULT_URL is invalid"),
            ),
            cache: dirs::cache_dir().map(|p| p.join("mesh-ftp")),
        }
    }

    /// Creates a [`MeSH`] instance with the default configuration.
    ///
    /// This is a convenience method that calls [`MeSH::default`].
    pub fn new() -> Self {
        Default::default()
    }

    streaming_fetch_method!(
        method = descriptor,
        process_fn = process_descriptor,
        record = DescriptorRecord,
        regex = r#"href="(desc[^"]+\.xml)""#,
        label = "Descriptor",
        error = MeSHError,
        err_missing = MeSHError::MissingFile,
    );

    streaming_fetch_method!(
        method = supplemental,
        process_fn = process_supplemental,
        record = SupplementalRecord,
        regex = r#"href="(supp[^"]+\.xml)""#,
        label = "Supplemental",
        error = MeSHError,
        err_missing = MeSHError::MissingFile,
    );

    streaming_fetch_method!(
        method = qualifier,
        process_fn = process_qualifier,
        record = QualifierRecord,
        regex = r#"href="(qual[^"]+\.xml)""#,
        label = "Qualifier",
        error = MeSHError,
        err_missing = MeSHError::MissingFile,
    );
}
