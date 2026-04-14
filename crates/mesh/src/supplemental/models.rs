//! Typed Rust representation of the NLM MeSH Supplemental Record Set XML
//! format.

use serde::{Deserialize, Serialize};

use crate::descriptor::models::{
    ConceptList, DescriptorReference, Name, NormalDate,
    PharmacologicalActionList, PreviousIndexingList, QualifierReference,
};

/// A single Supplementary Concept Record (SCR).
///
/// SCRs represent concepts — primarily chemicals and drugs — that are
/// automatically mapped to one or more MeSH Descriptor/Qualifier pairs when
/// indexing citations in PubMed. They are not part of the main MeSH
/// hierarchy but complement it.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SupplementalRecord {
    /// Broad category of the SCR. Determines how the record is used during
    /// indexing.
    #[serde(rename = "@SCRClass")]
    pub class: Option<String>,

    /// Alphanumeric unique identifier beginning with `C` (e.g. `"C000001"`).
    #[serde(rename = "SupplementalRecordUI")]
    pub ui: String,

    /// Canonical human-readable name of this SCR (e.g. `"calcimycin"`).
    #[serde(rename = "SupplementalRecordName")]
    pub name: Name,

    /// Date this record was first entered in the MeSH data-entry system.
    #[serde(rename = "DateIntroduced")]
    pub date_introduced: NormalDate,

    /// Date this record was last modified. `None` if never revised after
    /// creation.
    #[serde(rename = "LastUpdated")]
    pub date_updated: Option<NormalDate>,

    /// Free-text scope note or editorial comment about this concept, visible
    /// to catalogers and end users (distinct from an indexer annotation).
    #[serde(rename = "Note")]
    pub note: Option<String>,

    /// Number of PubMed citations indexed with this SCR. Automatically
    /// incremented monthly by NLM.
    #[serde(rename = "Frequency")]
    pub frequency: Option<u32>,

    /// MeSH Descriptors that were used to index this concept in earlier years
    /// before a dedicated SCR was created.
    #[serde(rename = "PreviousIndexingList")]
    pub previous_indexing: Option<PreviousIndexingList>,

    /// One or more Descriptor (optionally plus Qualifier) pairs that PubMed
    /// automatically attaches to a citation whenever this SCR is used to
    /// index it. At least one entry is required.
    #[serde(rename = "HeadingMappedToList")]
    pub heading_maps: Option<HeadingMappedToList>,

    /// Additional Descriptor/Qualifier/Chemical combinations used for
    /// supplementary indexing of citations with this SCR.
    #[serde(rename = "IndexingInformationList")]
    pub indexing_informations: Option<IndexingInformationList>,

    /// MeSH Descriptors that represent the pharmacological actions of this
    /// substance (e.g. "Anti-Bacterial Agents"). Only present for chemicals.
    #[serde(rename = "PharmacologicalActionList")]
    pub pharmacological_actions: Option<PharmacologicalActionList>,

    /// Bibliographic sources (journals or databases) where this concept was
    /// first reported or is documented.
    #[serde(rename = "SourceList")]
    pub sources: Option<SourceList>,

    /// One or more concepts that define the semantic content of this SCR.
    #[serde(rename = "ConceptList")]
    pub concepts: ConceptList,
}

/// Uniquely identifies an SCR.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SupplementalReference {
    /// Alphanumeric unique identifier beginning with `C` (e.g. `"C000001"`).
    #[serde(rename = "SupplementalRecordUI")]
    pub ui: String,

    /// Canonical human-readable name of this SCR (e.g. `"calcimycin"`).
    #[serde(rename = "SupplementalRecordName")]
    pub name: Name,
}

/// Descriptor (optionally plus Qualifier) pairs that PubMed automatically
/// attaches
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HeadingMappedToList {
    /// Map element
    #[serde(rename = "HeadingMappedTo")]
    pub items: Vec<HeadingMap>,
}

/// A Descriptor (optionally combined with a Qualifier).
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HeadingMap {
    /// The descriptor
    #[serde(rename = "DescriptorReferredTo")]
    pub descriptor: DescriptorReference,

    /// Qualifier that further specifies the mapping.
    /// `None` means the mapping is to the Descriptor alone.
    #[serde(rename = "QualifierReferredTo")]
    pub qualifier: Option<QualifierReference>,
}

/// Additional Descriptor/Qualifier/Chemical combinations.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IndexingInformationList {
    /// Indexing information element
    #[serde(rename = "IndexingInformation")]
    pub items: Vec<IndexingInformation>,
}

/// Additional Descriptor, Qualifier, or Chemical context used when indexing
/// citations with this SCR.
///
/// All three fields are optional; in practice at least one is always present.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IndexingInformation {
    /// A related Descriptor that provides further indexing specificity.
    #[serde(rename = "DescriptorReferredTo")]
    pub descriptor: Option<DescriptorReference>,

    /// A related Qualifier that provides further indexing specificity.
    #[serde(rename = "QualifierReferredTo")]
    pub qualifier: Option<QualifierReference>,

    /// A related SCR (chemical) that provides further indexing specificity.
    #[serde(rename = "ChemicalReferredTo")]
    pub chemical: Option<SupplementalReference>,
}

/// Wrapper element around a list of bibliographic sources.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SourceList {
    /// Bibliographic sources
    #[serde(rename = "Source", default)]
    pub items: Vec<String>,
}
