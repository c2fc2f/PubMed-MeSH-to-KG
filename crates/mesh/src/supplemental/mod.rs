//! Supplemental MeSH Data Types and Deserialization.
//!
//! This module provides a comprehensive suite of Rust data structures
//! designed to parse, serialize, and deserialize MeSH Supplemental XML
//! document structures. It utilizes [`serde`] to map complex, deeply nested
//! XML nodes into strongly typed enums and structs.

use crate::{
    error::MeSHError, streaming_processor, streaming_set_seed,
    supplemental::models::SupplementalRecord,
};

pub mod models;

streaming_set_seed!(
    seed = SupplementalRecordSetSeed,
    record = SupplementalRecord,
    set = "SupplementalRecordSet",
    item = "SupplementalRecord",
);

streaming_processor!(
    deserialize_fn = deserialize_supplemental,
    process_fn = process_supplemental,
    seed = SupplementalRecordSetSeed,
    record = SupplementalRecord,
    error = MeSHError,
    err_parsing = MeSHError::Parsing,
    err_cache = MeSHError::Cache,
    err_request = MeSHError::Request,
);
