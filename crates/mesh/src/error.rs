//! Error types for MeSH operations.
//!
//! This module defines [`MeSHError`], which encapsulates all possible
//! failures that can occur when communicating with the NLM FTP or
//! parsing dataset information.

use thiserror::Error;

#[cfg(feature = "debug_path")]
/// Type alias for parsing errors with path tracking enabled.
type ParseError = serde_path_to_error::Error<quick_xml::DeError>;

#[cfg(not(feature = "debug_path"))]
/// Type alias for standard XML parsing errors.
type ParseError = quick_xml::DeError;

/// Errors that can occur when interacting with the MeSH dataset.
#[derive(Error, Debug)]
pub enum MeSHError {
    /// An error occurred during an HTTP request or while processing
    /// the network response.
    ///
    /// This typically includes connection timeouts, DNS resolution
    /// failures, or invalid status codes returned by the NLM server.
    #[error("Network request failed")]
    Request(#[from] reqwest::Error),

    /// An error occurred while parsing the XML content.
    ///
    /// This variant wraps failures from the [`quick_xml`] deserializer,
    /// such as malformed XML syntax, unexpected tags, or data that
    /// does not conform to the expected MeSH schema.
    #[error("XML parsing failed at path")]
    Parsing(#[from] ParseError),

    /// An error occurred while reading from or writing to the local cache.
    ///
    /// This typically happens if the application lacks file system
    /// permissions, the disk is full, or the generated cache directory path
    /// is invalid.
    #[error("Cache storage operation failed")]
    Cache(#[from] std::io::Error),

    /// The requested file was not found on the NLM FTP server.
    ///
    /// This occurs when the expected file version is missing from the remote
    /// repository.
    #[error("{0} file not found on the FTP server")]
    MissingFile(String),
}
