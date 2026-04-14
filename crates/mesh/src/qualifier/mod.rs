//! Qualifier MeSH Data Types and Deserialization.
//!
//! This module provides a comprehensive suite of Rust data structures
//! designed to parse, serialize, and deserialize MeSH Qualifier XML
//! document structures. It utilizes [`serde`] to map complex, deeply nested
//! XML nodes into strongly typed enums and structs.

use crate::{
    error::MeSHError, qualifier::models::QualifierRecord, streaming_processor,
    streaming_set_seed,
};

pub mod models;

streaming_set_seed!(
    seed = QualifierRecordSetSeed,
    record = QualifierRecord,
    set = "QualifierRecordSet",
    item = "QualifierRecord",
);

streaming_processor!(
    deserialize_fn = deserialize_qualifier,
    process_fn = process_qualifier,
    seed = QualifierRecordSetSeed,
    record = QualifierRecord,
    error = MeSHError,
    err_parsing = MeSHError::Parsing,
    err_cache = MeSHError::Cache,
    err_request = MeSHError::Request,
);
