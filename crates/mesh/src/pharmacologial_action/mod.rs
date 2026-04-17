//! Pharmacological Action MeSH Data Types and Deserialization.
//!
//! This module provides a comprehensive suite of Rust data structures
//! designed to parse, serialize, and deserialize MeSH Pharmacological Action
//! XML document structures. It utilizes [`serde`] to map complex, deeply
//! nested XML nodes into strongly typed enums and structs.

use crate::{
    error::MeSHError,
    pharmacologial_action::models::PharmacologicalActionRecord,
    streaming_processor, streaming_set_seed,
};

pub mod models;

streaming_set_seed!(
    seed = PharmacologicalActionRecordSetSeed,
    record = PharmacologicalActionRecord,
    set = "PharmacologicalActionRecordSet",
    item = "PharmacologicalActionRecord",
);

streaming_processor!(
    deserialize_fn = deserialize_pharmacological_action,
    process_fn = process_pharmacological_action,
    seed = PharmacologicalActionRecordSetSeed,
    record = PharmacologicalActionRecord,
    error = MeSHError,
    err_parsing = MeSHError::Parsing,
    err_cache = MeSHError::Cache,
    err_request = MeSHError::Request,
);
