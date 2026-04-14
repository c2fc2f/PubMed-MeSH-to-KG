//! Typed Rust representation of the NLM MeSH Qualifier Record Set XML
//! format.

use serde::{Deserialize, Serialize};

use crate::descriptor::models::{
    ConceptList, Name, NormalDate, TreeNumberList,
};

/// A single MeSH Qualifier (also called a subheading or topical qualifier).
///
/// Qualifiers are combined with Descriptor Main Headings to provide more
/// specific indexing of biomedical citations (e.g. `Neoplasms/diagnosis`).
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct QualifierRecord {
    /// Stable MeSH qualifier identifier (e.g. `"Q000008"`).
    #[serde(rename = "QualifierUI")]
    pub ui: String,

    /// Display name of the referenced qualifier.
    #[serde(rename = "QualifierName")]
    pub name: Name,

    /// Date this record was first entered in the MeSH data-entry system.
    #[serde(rename = "DateIntroduced")]
    pub date_introduced: NormalDate,

    /// Date this record was last modified. `None` if never revised after
    /// creation.
    #[serde(rename = "LastUpdated")]
    pub date_updated: Option<NormalDate>,

    /// Free-text guidance for indexers and catalogers about correct usage of
    /// this qualifier in PubMed indexing.
    #[serde(rename = "Annotation")]
    pub annotation: Option<String>,

    /// Historical notes about the qualifier (name changes, scope changes,
    /// etc.).
    #[serde(rename = "HistoryNote")]
    pub history_note: Option<String>,

    /// Notes intended for online database users rather than indexers.
    #[serde(rename = "OnlineNote")]
    pub online_note: Option<String>,

    /// Hierarchical tree numbers that locate this qualifier within the MeSH
    /// tree structure (e.g. `"Q01.800"`).
    #[serde(rename = "TreeNumberList")]
    pub tree_numbers: Option<TreeNumberList>,

    /// One or more concepts that make up the semantic content of this
    /// qualifier. Always contains at least the preferred concept.
    #[serde(rename = "ConceptList")]
    pub concepts: ConceptList,
}
