//! # PubMed
//!
//! A high-performance, asynchronous Rust client for managing and processing
//! the PubMed baseline dataset.
//!
//! This library provides a structured way to interact with the NCBI FTP
//! servers, allowing users to discover, download, and iterate through the
//! massive collection of `pubmedXXnXXXX.xml` files. It is designed with
//! performance and local caching in mind to facilitate large-scale
//! bioinformatics data ingestion.
//!
//! ## Core Features
//!
//! * **Asynchronous API**: Built on [`reqwest`] and [`tokio`] for efficient
//!   network I/O.
//! * **Automated Discovery**: Automatically parses the NCBI baseline index to
//!   identify available XML chunks.
//! * **Configurable Caching**: Optional disk-based caching to minimize
//!   redundant downloads and respect NCBI bandwidth.
//! * **Builder Pattern**: Flexible configuration for custom mirrors, specific
//!   HTTP clients, or custom cache paths.
//!
//! ## Getting Started
//!
//! The primary entry point is the [`PubMed`] struct. You can use the default
//! configuration or customize it using the [`PubMedBuilder`].
//!
//! ### Basic Usage
//!
//! ```rust
//! use pubmed::PubMed;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Initialize with default settings (auto-resolves cache directory)
//!     let api = PubMed::new();
//!
//!     // Get the count of available data chunks
//!     let count = api.fetch_chunks_count().await?;
//!     println!("Found {} baseline chunks.", count);
//!
//!     // Stream chunks for processing
//!     let mut chunks = api.chunks().await?;
//!     
//!     Ok(())
//! }
//! ```
//!
//! ### Custom Configuration
//!
//! If you need to use a specific mirror or a dedicated cache directory:
//!
//! ```rust
//! use pubmed::PubMed;
//! use std::path::PathBuf;
//! use reqwest::Url;
//!
//! #[tokio::main]
//! async fn main() {
//!     let api = PubMed::builder()
//!         .base_url(Url::parse("https://my-mirror.example.com/pubmed/").unwrap())
//!         .cache(Some(PathBuf::from("./local_cache")))
//!         .build();
//! }
//! ```
//!
//! ## Modules
//!
//! * [`chunks`]: Logic for handling the collection and streaming of XML
//!   files.
//! * [`error`]: Custom error types for network and parsing failures.

pub mod chunks;
pub mod error;

use std::{collections::VecDeque, path::PathBuf, sync::LazyLock};

use regex::Regex;
use reqwest::Url;

use crate::{chunks::Chunks, error::PubMedError};

/// The default NCBI FTP URL
const DEFAULT_URL: &str = "https://ftp.ncbi.nlm.nih.gov/pubmed/baseline/";

/// A client for handling operations on the PubMed dataset.
///
/// This struct manages the connection to the NCBI servers and stores
/// the configuration required for dataset retrieval.
pub struct PubMed {
    /// The internal HTTP client used for making requests to the NCBI FTP.
    client: reqwest::Client,

    /// The resolved base URL (e.g., the default NCBI URL or a custom mirror).
    base_url: Url,

    /// The directory where downloaded `pubmedXXnXXXX.xml` files will be
    /// cached. If [`None`], caching is disabled and files are kept in memory.
    ///
    /// # Default Behavior
    ///
    /// When using [`PubMed::default()`], this automatically resolves to a
    /// `pm2kg`subdirectory within the system's standard cache location
    /// (determined via [`dirs::cache_dir`]).
    cache: Option<PathBuf>,
}

/// A [`PubMedBuilder`] can be used to create a [`PubMed`] with custom
/// configuration.
#[derive(Default)]
pub struct PubMedBuilder {
    /// The HTTP client used for making requests.
    /// If [`None`], a default [`reqwest::Client`] is created during
    /// [`PubMedBuilder::build()`].
    client: Option<reqwest::Client>,

    /// The base URL for the PubMed dataset.
    /// If [`None`], the default NCBI FTP URL is used during
    /// [`PubMedBuilder::build()`].
    base_url: Option<Url>,

    /// The directory where downloaded `pubmedXXnXXXX.xml` files will be
    /// cached. If [`None`], caching is disabled and files are kept in memory.
    cache: Option<PathBuf>,
}

impl PubMedBuilder {
    /// Creates a [`PubMedBuilder`] to configure a [`PubMed`].
    pub fn new() -> Self {
        Default::default()
    }

    /// Sets the HTTP client for the [`PubMed`] instance.
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

    /// Sets the base URL for the PubMed API.
    ///
    /// If not provided, the default NCBI FTP URL will be used during the
    /// build process.
    pub fn base_url(self, url: Url) -> Self {
        Self {
            client: self.client,
            base_url: Some(url),
            cache: self.cache,
        }
    }

    /// Sets the cache directory for storing downloaded `pubmedXXnXXXX.xml`
    /// files.
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

    /// Returns a [`PubMed`] that uses this [`PubMedBuilder`] configuration.
    ///
    /// # Panics
    ///
    /// Panics if no base URL provided and the default URL is not valid.
    pub fn build(self) -> PubMed {
        PubMed {
            client: self.client.unwrap_or_default(),
            base_url: self.base_url.unwrap_or_else(|| {
                Url::parse(DEFAULT_URL)
                    .expect("Hardcoded DEFAULT_URL is invalid")
            }),
            cache: self.cache,
        }
    }
}

impl Default for PubMed {
    fn default() -> Self {
        Self {
            client: Default::default(),
            base_url: Url::parse(DEFAULT_URL)
                .expect("Hardcoded DEFAULT_URL is invalid"),
            cache: dirs::cache_dir().map(|p| p.join("pm2kg")),
        }
    }
}

/// A regular expression used to extract `.xml.gz` file paths from HTML `href`
/// attributes.
///
/// It captures the URL/path inside the quotes into the first capture group.
///
/// # Panics
///
/// Panics at runtime during the first access if the regex pattern is invalid.
static HREF_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"href="([^"]+\.xml\.gz)""#)
        .expect("Invalid regex pattern (href)")
});

impl PubMed {
    /// Creates a [`PubMedBuilder`] to configure a [`PubMed`].
    ///
    /// # Panics
    ///
    /// Panics if the default URL is not valid.
    pub fn builder() -> PubMedBuilder {
        PubMedBuilder {
            client: Default::default(),
            base_url: Some(
                Url::parse(DEFAULT_URL)
                    .expect("Hardcoded DEFAULT_URL is invalid"),
            ),
            cache: dirs::cache_dir().map(|p| p.join("pm2kg")),
        }
    }

    /// Creates a [`PubMed`] instance with the default configuration.
    ///
    /// This is a convenience method that calls [`PubMed::default`].
    pub fn new() -> Self {
        Default::default()
    }

    /// Returns the number of XML chunks available at the base URL.
    ///
    /// # Errors
    ///
    /// Returns [`PubMedError::Request`] if the network request fails, the
    /// server returns an error status, or the response body is not valid
    /// UTF-8.
    pub async fn fetch_chunks_count(&self) -> Result<usize, PubMedError> {
        let content: String = self
            .client
            .get(self.base_url.as_str())
            .send()
            .await?
            .error_for_status()?
            .text()
            .await?;

        Ok(HREF_RE.captures_iter(&content).count())
    }

    /// Returns an iterable [`Chunks`] handle for the XML data in the
    /// baseline. The resulting [`Chunks`] object can be used to stream and
    /// parse the actual XML content.
    ///
    /// # Errors
    ///
    /// Returns [`PubMedError::Request`] if the network request fails, the
    /// server returns an error status, or the response body is not valid
    /// UTF-8.
    pub async fn chunks(&self) -> Result<Chunks, PubMedError> {
        let content: String = self
            .client
            .get(self.base_url.as_str())
            .send()
            .await?
            .error_for_status()?
            .text()
            .await?;

        let files: VecDeque<String> = HREF_RE
            .captures_iter(&content)
            .map(|cap| cap[1].to_string())
            .collect();

        Ok(Chunks::new(self, files))
    }
}
