//! Typed Rust representation of the NLM MeSH Descriptor Record Set XML
//! format.

use serde::{Deserialize, Serialize};

/// A single MeSH descriptor.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "DescriptorRecord")]
pub struct DescriptorRecord {
    /// Numeric class of this descriptor
    #[serde(rename = "@DescriptorClass")]
    pub class: Option<String>,

    /// Stable unique MeSH identifier for this descriptor (e.g. `"D000001"`).
    #[serde(rename = "DescriptorUI")]
    pub ui: String,

    /// Preferred English label for this descriptor.
    #[serde(rename = "DescriptorName")]
    pub name: Name,

    /// Date this record was last modified.
    #[serde(rename = "LastUpdated")]
    pub last_updated: NormalDate,

    /// Date this descriptor was first introduced into MeSH
    #[serde(rename = "DateIntroduced")]
    pub date_introduced: NormalDate,

    /// Legacy creation date(s) carried over from earlier records
    #[serde(rename = "DateCreated", default)]
    pub date_created: Vec<NormalDate>,

    /// Sub-headings (qualifiers) that may be combined with this descriptor.
    #[serde(rename = "AllowableQualifiersList")]
    pub allowable_qualifiers_list: Option<AllowableQualifiersList>,

    /// Free-text cataloging guidance for indexers.
    #[serde(rename = "Annotation")]
    pub annotation: Option<String>,

    /// Narrative history of how this heading has been used over time
    /// (`<HistoryNote>`).
    #[serde(rename = "HistoryNote")]
    pub history_note: Option<String>,

    /// NLM classification number mapping this descriptor to the NLM
    /// classification scheme.
    #[serde(rename = "NLMClassificationNumber")]
    pub nlm_classification_number: Option<String>,

    /// Note intended for online catalog display.
    #[serde(rename = "OnlineNote")]
    pub online_note: Option<String>,

    /// Note made available to end users through public MeSH interfaces
    #[serde(rename = "PublicMeSHNote")]
    pub public_mesh_note: Option<String>,

    /// Headings that were used to index this concept in earlier MeSH editions
    #[serde(rename = "PreviousIndexingList")]
    pub previous_indexing: Option<PreviousIndexingList>,

    /// Descriptor–qualifier combinations that should be mapped to a different
    /// heading.
    #[serde(rename = "EntryCombinationList")]
    pub entry_combination: Option<EntryCombinationList>,

    /// Related descriptors that indexers should also consider
    #[serde(rename = "SeeRelatedList")]
    pub see_related_list: Option<SeeRelatedList>,

    /// Free-text cross-reference hint pointing to lexically similar
    /// descriptors
    #[serde(rename = "ConsiderAlso")]
    pub consider_also: Option<String>,

    /// Descriptors representing pharmacological actions of this substance
    #[serde(rename = "PharmacologicalActionList")]
    pub pharmacological_action: Option<PharmacologicalActionList>,

    /// MeSH tree location codes for this descriptor.
    #[serde(rename = "TreeNumberList")]
    pub tree_numbers: Option<TreeNumberList>,

    /// All concepts.
    #[serde(rename = "ConceptList")]
    pub concepts: ConceptList,
}

/// A calendar date expressed as three separate text elements.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NormalDate {
    /// calendar year.
    #[serde(rename = "Year")]
    pub year: i32,

    /// month number.
    #[serde(rename = "Month")]
    pub month: u8,

    /// day of month.
    #[serde(rename = "Day")]
    pub day: u8,
}

/// Wrapper for a entity display label.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Name {
    /// The human-readable name string of the entity.
    #[serde(rename = "String")]
    pub value: String,
}

/// Lightweight reference to another qualifier by UI and name
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualifierReference {
    /// Stable MeSH qualifier identifier (e.g. `"Q000008"`).
    #[serde(rename = "QualifierUI")]
    pub ui: String,

    /// Display name of the referenced qualifier.
    #[serde(rename = "QualifierName")]
    pub name: Name,
}

/// Lightweight reference to another descriptor by UI and name
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DescriptorReference {
    /// Stable MeSH descriptor identifier (e.g. `"D000001"`).
    #[serde(rename = "DescriptorUI")]
    pub ui: String,

    /// Display name of the referenced descriptor.
    #[serde(rename = "DescriptorName")]
    pub name: Name,
}

/// List of qualifiers (sub-headings) permitted for a given descriptor
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllowableQualifiersList {
    /// One or more allowable qualifier entries.
    #[serde(rename = "AllowableQualifier")]
    pub items: Vec<AllowableQualifier>,
}

/// A single qualifier permitted for use with the parent descriptor
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllowableQualifier {
    /// Identity of the qualifier that may be applied.
    #[serde(rename = "QualifierReferredTo")]
    pub qualifier_referred_to: QualifierReference,

    /// Short two-letter abbreviation for the qualifier
    #[serde(rename = "Abbreviation")]
    pub abbreviation: String,
}

/// List of headings used to index this concept in earlier MeSH editions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreviousIndexingList {
    /// Free-text previous indexing strings, one per historical heading.
    #[serde(rename = "PreviousIndexing")]
    pub items: Vec<String>,
}

/// List of descriptor–qualifier mappings that must be replaced by an
/// alternative heading during indexing.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntryCombinationList {
    /// One or more entry combination rules.
    #[serde(rename = "EntryCombination")]
    pub items: Vec<EntryCombination>,
}

/// A single entry combination rule.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntryCombination {
    /// The input heading combination that triggers the mapping.
    #[serde(rename = "ECIN")]
    pub ecin: EcIn,

    /// The output heading combination that replaces the input.
    #[serde(rename = "ECOUT")]
    pub ecout: EcOut,
}

/// The "entry combination in" side of a mapping.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcIn {
    /// The descriptor portion of the disallowed combination.
    #[serde(rename = "DescriptorReferredTo")]
    pub descriptor_referred_to: DescriptorReference,

    /// The qualifier portion of the disallowed combination.
    #[serde(rename = "QualifierReferredTo")]
    pub qualifier_referred_to: QualifierReference,
}

/// The "entry combination out" side of a mapping.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcOut {
    /// The descriptor to use as the replacement heading.
    #[serde(rename = "DescriptorReferredTo")]
    pub descriptor_referred_to: DescriptorReference,

    /// An optional qualifier to attach to the replacement descriptor.
    #[serde(rename = "QualifierReferredTo")]
    pub qualifier_referred_to: Option<QualifierReference>,
}

/// List of descriptors that are related but not hierarchically linked
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeeRelatedList {
    /// One or more related descriptor references.
    #[serde(rename = "SeeRelatedDescriptor")]
    pub items: Vec<SeeRelatedDescriptor>,
}

/// A single "see also" cross-reference to another descriptor
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeeRelatedDescriptor {
    /// The related descriptor being pointed to.
    #[serde(rename = "DescriptorReferredTo")]
    pub descriptor_referred_to: DescriptorReference,
}

/// List of pharmacological action descriptors attributed to a substance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PharmacologicalActionList {
    /// One or more pharmacological action entries.
    #[serde(rename = "PharmacologicalAction")]
    pub items: Vec<PharmacologicalAction>,
}

/// A single pharmacological action associated with a substance descriptor
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PharmacologicalAction {
    /// The descriptor that names the pharmacological action.
    #[serde(rename = "DescriptorReferredTo")]
    pub descriptor_referred_to: DescriptorReference,
}

/// Set of hierarchical classification codes locating a descriptor in the MeSH
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TreeNumberList {
    /// Dot-separated tree numbers, e.g. `"A01.378.610"`.
    #[serde(rename = "TreeNumber")]
    pub items: Vec<String>,
}

/// Ordered collection of concepts grouped under a descriptor
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConceptList {
    /// All concepts belonging to this descriptor.
    #[serde(rename = "Concept")]
    pub items: Vec<Concept>,
}

/// A concept within a descriptor, grouping synonymous terms under a common
/// meaning.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Concept {
    /// Whether this is the preferred concept for the descriptor
    #[serde(rename = "@PreferredConceptYN")]
    pub preferred_concept_yn: YesNo,

    /// Stable MeSH concept identifier (e.g. `"M0000001"`).
    #[serde(rename = "ConceptUI")]
    pub ui: String,

    /// Display name of this concept.
    #[serde(rename = "ConceptName")]
    pub name: Name,

    /// Chemical Abstracts Service (CAS) Type 1 name for this substance
    #[serde(rename = "CASN1Name")]
    pub casn1_name: Option<String>,

    /// CAS registry numbers and other chemical identifiers for this concept
    #[serde(rename = "RegistryNumberList")]
    pub registry_number_list: Option<RegistryNumberList>,

    /// Definition or scope of this concept as used in MeSH indexing
    #[serde(rename = "ScopeNote")]
    pub scope_note: Option<String>,

    /// English-language scope note added by a non-English MeSH translator for
    /// reference purposes
    #[serde(rename = "TranslatorsEnglishScopeNote")]
    pub translators_english_scope_note: Option<String>,

    /// Scope note in the translator's own language
    #[serde(rename = "TranslatorsScopeNote")]
    pub translators_scope_note: Option<String>,

    /// Additional chemical registry numbers related to this concept
    #[serde(rename = "RelatedRegistryNumberList")]
    pub related_registry_number: Option<RelatedRegistryNumberList>,

    /// Semantic relationships between this concept and other concepts
    #[serde(rename = "ConceptRelationList")]
    pub concept_relations: Option<ConceptRelationList>,

    /// All terms that express this concept.
    #[serde(rename = "TermList")]
    pub terms: TermList,
}

/// List of primary chemical registry numbers for a concept
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryNumberList {
    /// One or more registry number strings.
    #[serde(rename = "RegistryNumber")]
    pub items: Vec<String>,
}

/// List of additional chemical registry numbers related to the concept
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelatedRegistryNumberList {
    /// One or more related registry number strings.
    #[serde(rename = "RelatedRegistryNumber")]
    pub items: Vec<String>,
}

/// Collection of semantic relationships between concepts within a descriptor
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConceptRelationList {
    /// One or more concept relation entries.
    #[serde(rename = "ConceptRelation")]
    pub items: Vec<ConceptRelation>,
}

/// Semantic relationship type between two concepts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RelationName {
    /// Narrower — the second concept is more specific than the first.
    NRW,
    /// Broader — the second concept is more general than the first.
    BRD,
    /// Related — the concepts are semantically related but neither is
    /// narrower nor broader.
    REL,
}

/// A directed semantic relationship between two concepts within a descriptor
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConceptRelation {
    /// The type of semantic relationship (attribute `RelationName`).
    ///
    /// The DTD marks this `#IMPLIED`, so it may be absent.
    #[serde(rename = "@RelationName")]
    pub name: Option<RelationName>,

    /// MeSH UI of the first (source) concept in the relationship.
    #[serde(rename = "Concept1UI")]
    pub concept1: String,

    /// MeSH UI of the second (target) concept in the relationship.
    #[serde(rename = "Concept2UI")]
    pub concept2: String,
}

/// Ordered collection of terms that express a single concept..
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TermList {
    /// All terms belonging to this concept.
    #[serde(rename = "Term")]
    pub items: Vec<Term>,
}

/// Lexical category of a MeSH term
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LexicalTag {
    /// Abbreviation (e.g. `"MRI"`).
    ABB,
    /// Abbreviation with expansion (embedded expansion of an abbreviation).
    ABX,
    /// Acronym (e.g. `"AIDS"`).
    ACR,
    /// Acronym with expansion.
    ACX,
    /// Eponym — named after a person or place (e.g. `"Hodgkin Disease"`).
    EPO,
    /// Lab/test number or code.
    LAB,
    /// Proper name.
    NAM,
    /// None of the above / unclassified.
    NON,
    /// Trade name / brand name.
    TRD,
    /// Free-text lexical entry (used for non-English MeSH).
    Frelex,
    /// Historical term no longer in active use.
    HIST,
}

/// A single lexical term within a concept.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Term {
    /// Whether this term is the preferred term for its parent concept
    #[serde(rename = "@ConceptPreferredTermYN")]
    pub concept_preferred_term_yn: YesNo,

    /// Whether this term is a permuted (inverted) form of another term
    #[serde(rename = "@IsPermutedTermYN")]
    pub is_permuted_term_yn: YesNo,

    /// Lexical category of this term
    #[serde(rename = "@LexicalTag")]
    pub lexical_tag: LexicalTag,

    /// Whether this term is the single preferred term
    #[serde(rename = "@RecordPreferredTermYN")]
    pub record_preferred_term_yn: YesNo,

    /// Stable MeSH term identifier (e.g. `"T000001"`).
    #[serde(rename = "TermUI")]
    pub term_ui: String,

    /// The actual term string as it appears in MeSH (e.g.
    /// `"Artificial Heart"`).
    #[serde(rename = "String")]
    pub string: String,

    /// Date(s) on which this term was created.
    #[serde(rename = "DateCreated", default)]
    pub date_created: Vec<NormalDate>,

    /// Short abbreviation for this term, if one exists
    #[serde(rename = "Abbreviation")]
    pub abbreviation: Option<String>,

    /// Alphabetically sortable version of the term string
    #[serde(rename = "SortVersion")]
    pub sort_version: Option<String>,

    /// Normalized entry form used for display in indexes and catalogs
    #[serde(rename = "EntryVersion")]
    pub entry_version: Option<String>,

    /// Thesauri or controlled vocabularies that also contain this term
    #[serde(rename = "ThesaurusIDlist")]
    pub thesaurus_id_list: Option<ThesaurusIdList>,

    /// Scope or usage note specific to this individual term
    #[serde(rename = "TermNote")]
    pub term_note: Option<String>,
}

/// List of external thesaurus identifiers that include a given term
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThesaurusIdList {
    /// One or more thesaurus identifier strings (e.g. `"FDA SRS"`).
    #[serde(rename = "ThesaurusID")]
    pub items: Vec<String>,
}

/// Boolean flag encoded as `"Y"` or `"N"` in XML attributes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum YesNo {
    /// `"Y"` — yes / true.
    Y,
    /// `"N"` — no / false.
    N,
}
