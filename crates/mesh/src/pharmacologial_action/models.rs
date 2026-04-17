//! Typed Rust representation of the NLM MeSH Pharmacological Action Record
//! Set XML format.

use serde::{Deserialize, Serialize};

use crate::descriptor::models::{DescriptorReference, Name};

/// A single pharmacological action, linking a MeSH descriptor
/// to the substances known to produce that action.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "PharmacologicalAction")]
pub struct PharmacologicalActionRecord {
    /// The MeSH descriptor that defines the pharmacological action (e.g.
    /// "Anti-Bacterial Agents").
    #[serde(rename = "DescriptorReferredTo")]
    pub descriptor_referred_to: DescriptorReference,

    /// The list of substances that exhibit this pharmacological action.
    #[serde(rename = "PharmacologicalActionSubstanceList")]
    pub substance_list: PharmacologicalActionSubstanceList,
}

/// Wrapper holding one or more substances sharing a pharmacological action.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "PharmacologicalActionSubstanceList")]
pub struct PharmacologicalActionSubstanceList {
    /// At least one substance is always present.
    #[serde(rename = "Substance")]
    pub substances: Vec<Substance>,
}

/// A chemical substance or drug record from the MeSH database.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "Substance")]
pub struct Substance {
    /// Unique record identifier for this substance (e.g. `"C000001"`).
    #[serde(rename = "RecordUI")]
    pub ui: String,

    /// Human-readable name of the substance.
    #[serde(rename = "RecordName")]
    pub name: Name,
}
