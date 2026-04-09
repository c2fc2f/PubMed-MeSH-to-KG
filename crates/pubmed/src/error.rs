//! Error types for PubMed operations.
//!
//! This module defines [`PubMedError`], which encapsulates all possible
//! failures that can occur when communicating with the NCBI FTP or
//! parsing dataset information.

use thiserror::Error;

#[cfg(feature = "debug_path")]
/// Type alias for parsing errors with path tracking enabled.
type ParseError = serde_path_to_error::Error<quick_xml::DeError>;

#[cfg(not(feature = "debug_path"))]
/// Type alias for standard XML parsing errors.
type ParseError = quick_xml::DeError;

/// Errors that can occur when interacting with the PubMed dataset.
#[derive(Error, Debug)]
pub enum PubMedError {
    /// An error occurred during an HTTP request or while processing
    /// the network response.
    ///
    /// This typically includes connection timeouts, DNS resolution
    /// failures, or invalid status codes returned by the NCBI server.
    #[error("Network request failed")]
    Request(#[from] reqwest::Error),

    /// An error occurred while parsing the XML content.
    ///
    /// This variant wraps failures from the [`quick_xml`] deserializer,
    /// such as malformed XML syntax, unexpected tags, or data that
    /// does not conform to the expected PubMed schema.
    #[error("XML parsing failed at path")]
    Parsing(#[from] ParseError),

    /// An error occurred while reading from or writing to the local cache.
    ///
    /// This typically happens if the application lacks file system
    /// permissions, the disk is full, or the generated cache directory path
    /// is invalid.
    #[error("Cache storage operation failed")]
    Cache(#[from] std::io::Error),
}
