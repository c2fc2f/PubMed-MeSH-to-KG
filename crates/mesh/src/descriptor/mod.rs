//! Descriptor MeSH Data Types and Deserialization.
//!
//! This module provides a comprehensive suite of Rust data structures
//! designed to parse, serialize, and deserialize MeSH Descriptor XML
//! document structures. It utilizes [`serde`] to map complex, deeply nested
//! XML nodes into strongly typed enums and structs.

use crate::{
    descriptor::models::DescriptorRecord, error::MeSHError,
    streaming_processor, streaming_set_seed,
};

pub mod models;

streaming_set_seed!(
    seed = DescriptorRecordSetSeed,
    record = DescriptorRecord,
    set = "DescriptorRecordSet",
    item = "DescriptorRecord",
);

streaming_processor!(
    deserialize_fn = deserialize_descriptor,
    process_fn = process_descriptor,
    seed = DescriptorRecordSetSeed,
    record = DescriptorRecord,
    error = MeSHError,
    err_parsing = MeSHError::Parsing,
    err_cache = MeSHError::Cache,
    err_request = MeSHError::Request,
);
