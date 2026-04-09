//! PubMed and MEDLINE XML Data Types and Deserialization.
//!
//! This module provides a comprehensive suite of Rust data structures
//! designed to parse, serialize, and deserialize PubMed and MEDLINE XML
//! document structures. It utilizes [`serde`] to map complex, deeply nested
//! XML nodes into strongly typed enums and structs, covering articles, books,
//! citations, author information, and more.
//!
//! Additionally, it integrates with the [`textml`] module to automatically
//! convert mixed-content text fields (containing inline HTML or MathML) into
//! Markdown.

pub mod textml;

use serde::{Deserialize, Serialize};
use textml::deserialize_to_markdown;

/// Represents a simple Yes/No boolean flag frequently used in PubMed XML
/// attributes.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Default)]
pub enum YN {
    /// Yes.
    #[default]
    Y,
    /// No.
    N,
}

/// Specifies the organization or group that owns the citation data.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Default)]
pub enum OwnerNlm {
    /// National Library of Medicine (default).
    #[default]
    NLM,
    /// National Aeronautics and Space Administration.
    NASA,
    /// Population Information Program.
    PIP,
    /// Kennedy Institute of Ethics.
    KIE,
    /// Health Services Research.
    HSR,
    /// History of Medicine Division.
    HMD,
    /// Not owned by NLM.
    NOTNLM,
}

/// Specifies the owner of a general note attached to a citation.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Default)]
pub enum OwnerNote {
    /// National Library of Medicine (default).
    #[default]
    NLM,
    /// National Aeronautics and Space Administration.
    NASA,
    /// Population Information Program.
    PIP,
    /// Kennedy Institute of Ethics.
    KIE,
    /// Health Services Research.
    HSR,
    /// History of Medicine Division.
    HMD,
}

/// Indicates the current stage of processing for a MEDLINE citation.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum MedlineCitationStatus {
    /// Fully indexed and completed citation.
    Completed,
    /// Citation is currently being processed.
    #[serde(rename = "In-Process")]
    InProcess,
    /// Record from a non-MEDLINE journal in PubMed.
    #[serde(rename = "PubMed-not-MEDLINE")]
    PubMedNotMedline,
    /// Citation is undergoing data review.
    #[serde(rename = "In-Data-Review")]
    InDataReview,
    /// Citation submitted directly by the publisher.
    Publisher,
    /// Fully indexed MEDLINE record.
    MEDLINE,
    /// Older MEDLINE record.
    OLDMEDLINE,
}

/// Describes the medium through which an article was published.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum ArticlePubModel {
    /// Published in print.
    Print,
    /// Published in print, followed by an electronic release.
    #[serde(rename = "Print-Electronic")]
    PrintElectronic,
    /// Published electronically.
    Electronic,
    /// Published electronically, followed by a print release.
    #[serde(rename = "Electronic-Print")]
    ElectronicPrint,
    /// Electronic publication bundled in an electronic collection.
    #[serde(rename = "Electronic-eCollection")]
    ElectronicECollection,
}

/// Identifies the specific type of article identifier.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum ArticleIdType {
    /// Digital Object Identifier.
    DOI,
    /// Publisher Item Identifier.
    PII,
    /// PubMed Central Publisher ID.
    PMCPID,
    /// PubMed Publisher ID.
    PMPID,
    /// PubMed Central ID.
    PMC,
    /// MEDLINE ID (legacy).
    MID,
    /// Serial Item and Contribution Identifier.
    SICI,
    /// PubMed unique identifier (default).
    #[default]
    PUBMED,
    /// MEDLINE unique identifier.
    MEDLINE,
    /// PubMed Central unique identifier.
    PMCID,
    /// PMC Book identifier.
    PMCBOOK,
    /// Book Accession ID.
    BOOKACCESSION,
}

/// Identifies the type of an electronic location identifier.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum EIdType {
    /// Digital Object Identifier.
    DOI,
    /// Publisher Item Identifier.
    PII,
}

/// Defines the format medium for an International Standard Serial Number
/// (ISSN).
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum IssnType {
    /// Electronic ISSN.
    Electronic,
    /// Print ISSN.
    Print,
}

/// Specifies the medium in which the cited journal issue was published.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum CitedMedium {
    /// Published on the Internet.
    Internet,
    /// Published in print.
    Print,
}

/// Represents the structural category of an abstract section based on NLM
/// guidelines.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum NlmCategory {
    /// Background or introduction section.
    Background,
    /// Objective or purpose of the study.
    Objective,
    /// Methodology and materials used.
    Methods,
    /// Findings and results.
    Results,
    /// Conclusions and implications.
    Conclusions,
    /// Unassigned or uncategorized section.
    Unassigned,
}

/// Identifies the type of relationship between the current citation and
/// another reference.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum CommentsCorrectionsRefType {
    /// Associated dataset.
    AssociatedDataset,
    /// Associated publication.
    AssociatedPublication,
    /// This article comments on another.
    CommentIn,
    /// Another article comments on this one.
    CommentOn,
    /// Corrected and republished elsewhere.
    CorrectedandRepublishedIn,
    /// Corrected and republished from another source.
    CorrectedandRepublishedFrom,
    /// Erratum published elsewhere.
    ErratumIn,
    /// This is an erratum for another article.
    ErratumFor,
    /// Expression of concern published elsewhere.
    ExpressionOfConcernIn,
    /// This is an expression of concern for another article.
    ExpressionOfConcernFor,
    /// Republished elsewhere.
    RepublishedIn,
    /// Republished from another source.
    RepublishedFrom,
    /// Retracted and republished elsewhere.
    RetractedandRepublishedIn,
    /// Retracted and republished from another source.
    RetractedandRepublishedFrom,
    /// Retraction published elsewhere.
    RetractionIn,
    /// This article is a retraction of another.
    RetractionOf,
    /// Update published elsewhere.
    UpdateIn,
    /// This article is an update of another.
    UpdateOf,
    /// Summary for patients published elsewhere.
    SummaryForPatientsIn,
    /// Original report published elsewhere.
    OriginalReportIn,
    /// Reprint published elsewhere.
    ReprintIn,
    /// This is a reprint of another article.
    ReprintOf,
    /// This article cites another.
    Cites,
}

/// Represents the publication status timeline events of an article.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum PubStatus {
    /// Manuscript received.
    Received,
    /// Manuscript accepted.
    Accepted,
    /// Published electronically.
    Epublish,
    /// Published in print.
    Ppublish,
    /// Manuscript revised.
    Revised,
    /// Ahead of print publication.
    Aheadofprint,
    /// Article retracted.
    Retracted,
    /// Included in an electronic collection.
    Ecollection,
    /// Available in PubMed Central.
    PMC,
    /// Under PMC review.
    PMCR,
    /// Available in PubMed.
    PubMed,
    /// Under PubMed review.
    PubMedR,
    /// Pre-MEDLINE status.
    PreMedline,
    /// Fully indexed in MEDLINE.
    Medline,
    /// Under MEDLINE review.
    MedlineR,
    /// Available in Entrez.
    Entrez,
    /// PubMed Central release date.
    #[serde(rename = "pmc-release")]
    PmcRelease,
}

/// Identifies whether a list contains authors or editors.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum AuthorListType {
    /// A list of contributing authors.
    Authors,
    /// A list of editors.
    Editors,
}

/// Specifies the organization that supplied a particular keyword list.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Default)]
pub enum KeywordListOwner {
    /// National Library of Medicine (default).
    #[default]
    NLM,
    /// Automated extraction by NLM.
    #[serde(rename = "NLM-AUTO")]
    NLMAUTO,
    /// National Aeronautics and Space Administration.
    NASA,
    /// Population Information Program.
    PIP,
    /// Kennedy Institute of Ethics.
    KIE,
    /// Not owned by NLM.
    NOTNLM,
    /// Department of Health and Human Services.
    HHS,
}

/// Indicates the category of a supplementary MeSH concept.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum SupplMeshNameType {
    /// Disease concept.
    Disease,
    /// Protocol concept.
    Protocol,
    /// Organism concept.
    Organism,
    /// Anatomical concept.
    Anatomy,
    /// Population concept.
    Population,
}

/// Categorizes abstracts provided by organizations other than NLM.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum OtherAbstractType {
    /// Association of American Medical Colleges.
    AAMC,
    /// AIDS-related abstract.
    AIDS,
    /// Kennedy Institute of Ethics.
    KIE,
    /// Population Information Program.
    PIP,
    /// NASA-related abstract.
    NASA,
    /// Provided by the publisher.
    Publisher,
    /// A plain language summary for general audiences.
    #[serde(rename = "plain-language-summary")]
    PlainLanguageSummary,
}

/// Identifies the source organization for an Other ID.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum OtherIdSource {
    /// National Aeronautics and Space Administration.
    NASA,
    /// Kennedy Institute of Ethics.
    KIE,
    /// Population Information Program.
    PIP,
    /// POP source.
    POP,
    /// ARPL source.
    ARPL,
    /// CPC source.
    CPC,
    /// IND source.
    IND,
    /// CPFH source.
    CPFH,
    /// CLML source.
    CLML,
    /// NRCBL source.
    NRCBL,
    /// National Library of Medicine.
    NLM,
    /// QCIM source.
    QCIM,
}

/// Specifies the structural type of a document location label.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum LocationLabelType {
    /// A part within a document.
    Part,
    /// A chapter within a document.
    Chapter,
    /// A section within a document.
    Section,
    /// An appendix to a document.
    Appendix,
    /// A figure label.
    Figure,
    /// A table label.
    Table,
    /// A breakout box label.
    #[serde(rename = "box_")]
    Box,
}

/// Specifies the conceptual type of a descriptor name.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum DescriptorNameType {
    /// Geographic descriptor.
    Geographic,
}

// ================================================================
// Root set elements
// ================================================================

/// Root element encapsulating a collection of PubMed articles and book
/// articles. Also includes an optional list of citations to be deleted.
///
/// NOTE: [`PubmedArticle`] and [`PubmedBookArticle`] elements are interleaved
/// in the XML but are flattened into two separate [`Vec`]s here. Insertion
/// order is not preserved across the different kinds.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename = "PubmedArticleSet")]
pub struct PubmedArticleSet {
    /// The collection of standard PubMed articles.
    #[serde(rename = "PubmedArticle", default)]
    pub articles: Vec<PubmedArticle>,

    /// The collection of PubMed book articles.
    #[serde(rename = "PubmedBookArticle", default)]
    pub book_articles: Vec<PubmedBookArticle>,

    /// An optional list of PMIDs marked for deletion.
    #[serde(rename = "DeleteCitation")]
    pub delete_citation: Option<DeleteCitation>,
}

/// Root element encapsulating a collection of Book Documents.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename = "BookDocumentSet")]
pub struct BookDocumentSet {
    /// The collection of book documents.
    #[serde(rename = "BookDocument", default)]
    pub documents: Vec<BookDocument>,

    /// An optional list of book document PMIDs marked for deletion.
    #[serde(rename = "DeleteDocument")]
    pub delete_document: Option<DeleteDocument>,
}

/// Root element exclusively containing PubMed book articles.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename = "PubmedBookArticleSet")]
pub struct PubmedBookArticleSet {
    /// The collection of book articles.
    #[serde(rename = "PubmedBookArticle", default)]
    pub articles: Vec<PubmedBookArticle>,
}

// ================================================================
// Document-level elements
// ================================================================

/// Represents a single standard PubMed article record.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename = "PubmedArticle")]
pub struct PubmedArticle {
    /// The core MEDLINE citation data for the article.
    #[serde(rename = "MedlineCitation")]
    pub medline_citation: MedlineCitation,

    /// Additional data tracking the article's history in PubMed.
    #[serde(rename = "PubmedData")]
    pub pubmed_data: Option<PubmedData>,
}

/// Represents a single book article or chapter record in PubMed.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename = "PubmedBookArticle")]
pub struct PubmedBookArticle {
    /// The core document data for the book/chapter.
    #[serde(rename = "BookDocument")]
    pub book_document: BookDocument,

    /// Additional data tracking the book's history in PubMed.
    #[serde(rename = "PubmedBookData")]
    pub pubmed_book_data: Option<PubmedBookData>,
}

/// Contains detailed bibliographic information for a book or book chapter.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename = "BookDocument")]
pub struct BookDocument {
    /// The PubMed Unique Identifier for the book document.
    #[serde(rename = "PMID")]
    pub pmid: Pmid,

    /// Associated article identifiers.
    #[serde(rename = "ArticleIdList")]
    pub article_id_list: ArticleIdList,

    /// Details of the book itself.
    #[serde(rename = "Book")]
    pub book: Book,

    /// Structural location labels within the book.
    #[serde(rename = "LocationLabel", default)]
    pub location_labels: Vec<LocationLabel>,

    /// The title of the specific article or chapter.
    #[serde(rename = "ArticleTitle")]
    pub article_title: Option<Text>,

    /// The title of the article/chapter in its original language.
    #[serde(rename = "VernacularTitle")]
    pub vernacular_title: Text,

    /// Page numbers for the document.
    #[serde(rename = "Pagination")]
    pub pagination: Option<Pagination>,

    /// The language(s) the document is written in.
    #[serde(rename = "Language", default)]
    pub languages: Vec<String>,

    /// The authors contributing to the document.
    #[serde(rename = "AuthorList", default)]
    pub author_lists: Vec<AuthorList>,

    /// A list of investigators or collaborators.
    #[serde(rename = "InvestigatorList")]
    pub investigator_list: Option<InvestigatorList>,

    /// Categorizations of the publication's type.
    #[serde(rename = "PublicationType", default)]
    pub publication_types: Vec<PublicationType>,

    /// The abstract text of the document.
    #[serde(rename = "Abstract")]
    pub abstract_: Option<Abstract>,

    /// Structural sections within the document abstract/content.
    #[serde(rename = "Sections")]
    pub sections: Option<Sections>,

    /// Keyword lists provided for indexing.
    #[serde(rename = "KeywordList", default)]
    pub keyword_lists: Vec<KeywordList>,

    /// The date the document was contributed to the database.
    #[serde(rename = "ContributionDate")]
    pub contribution_date: Option<DateYMD>,

    /// The date the document was last revised.
    #[serde(rename = "DateRevised")]
    pub date_revised: Option<DateYMD>,

    /// Grants supporting the research detailed in the document.
    #[serde(rename = "GrantList")]
    pub grant_list: Option<GrantList>,

    /// General categorized item lists associated with the document.
    #[serde(rename = "ItemList", default)]
    pub item_lists: Vec<ItemList>,

    /// Bibliographic references cited by the document.
    #[serde(rename = "ReferenceList", default)]
    pub reference_lists: Vec<ReferenceList>,
}

/// Contains a list of standard citations (PMIDs) marked for deletion.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename = "DeleteCitation")]
pub struct DeleteCitation {
    /// 1 or more PMIDs targeted for removal.
    #[serde(rename = "PMID")]
    pub pmids: Vec<Pmid>,
}

/// Contains a list of book documents (PMIDs) marked for deletion.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename = "DeleteDocument")]
pub struct DeleteDocument {
    /// A list of PMIDs targeted for removal.
    #[serde(rename = "PMID", default)]
    pub pmids: Vec<Pmid>,
}

// ================================================================
// Sub-document wrapper elements
// ================================================================

/// Represents the core bibliographical and indexing data for a MEDLINE
/// article.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename = "MedlineCitation")]
pub struct MedlineCitation {
    /// The owner of the citation data.
    #[serde(rename = "@Owner", default)]
    pub owner: OwnerNlm,

    /// The current processing status of the citation.
    #[serde(rename = "@Status")]
    pub status: MedlineCitationStatus,

    /// Identifier for the citation version.
    #[serde(rename = "@VersionID")]
    pub version_id: Option<String>,

    /// Date the version was generated.
    #[serde(rename = "@VersionDate")]
    pub version_date: Option<String>,

    /// The method used to index the citation (e.g., automated vs. human).
    #[serde(rename = "@IndexingMethod")]
    pub indexing_method: Option<String>,

    /// The unique PubMed Identifier.
    #[serde(rename = "PMID")]
    pub pmid: Pmid,

    /// Date the citation was completed.
    #[serde(rename = "DateCompleted")]
    pub date_completed: Option<DateYMD>,

    /// Date the citation was last revised.
    #[serde(rename = "DateRevised")]
    pub date_revised: Option<DateYMD>,

    /// Core information about the published article.
    #[serde(rename = "Article")]
    pub article: Article,

    /// Information about the journal containing the article.
    #[serde(rename = "MedlineJournalInfo")]
    pub medline_journal_info: MedlineJournalInfo,

    /// List of chemical substances discussed in the article.
    #[serde(rename = "ChemicalList")]
    pub chemical_list: Option<ChemicalList>,

    /// Supplementary Medical Subject Headings (MeSH).
    #[serde(rename = "SupplMeshList")]
    pub suppl_mesh_list: Option<SupplMeshList>,

    /// Subsets of MEDLINE the citation belongs to.
    #[serde(rename = "CitationSubset", default)]
    pub citation_subsets: Vec<String>,

    /// Relationships to other publications (errata, retractions, etc.).
    #[serde(rename = "CommentsCorrectionsList")]
    pub comments_corrections_list: Option<CommentsCorrectionsList>,

    /// List of gene symbols reported in the article.
    #[serde(rename = "GeneSymbolList")]
    pub gene_symbol_list: Option<GeneSymbolList>,

    /// Medical Subject Headings (MeSH) indexing the article.
    #[serde(rename = "MeshHeadingList")]
    pub mesh_heading_list: Option<MeshHeadingList>,

    /// Number of references cited by the article.
    #[serde(rename = "NumberOfReferences")]
    pub number_of_references: Option<String>,

    /// Specific individuals acting as subjects of the article.
    #[serde(rename = "PersonalNameSubjectList")]
    pub personal_name_subject_list: Option<PersonalNameSubjectList>,

    /// Additional identifiers assigned by other systems.
    #[serde(rename = "OtherID", default)]
    pub other_ids: Vec<OtherId>,

    /// Alternative abstracts provided by external organizations.
    #[serde(rename = "OtherAbstract", default)]
    pub other_abstracts: Vec<OtherAbstract>,

    /// Keyword lists for search indexing.
    #[serde(rename = "KeywordList", default)]
    pub keyword_lists: Vec<KeywordList>,

    /// Conflict of interest statement.
    #[serde(rename = "CoiStatement")]
    pub coi_statement: Option<Text>,

    /// Identifies associated space flight missions.
    #[serde(rename = "SpaceFlightMission", default)]
    pub space_flight_missions: Vec<String>,

    /// List of investigators or non-author contributors.
    #[serde(rename = "InvestigatorList", default)]
    pub investigator_lists: Vec<InvestigatorList>,

    /// General cataloging notes.
    #[serde(rename = "GeneralNote", default)]
    pub general_notes: Vec<GeneralNote>,
}

/// Contains PubMed-specific metadata surrounding a standard article.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename = "PubmedData")]
pub struct PubmedData {
    /// Historical dates associated with the publication and indexing
    /// lifecycle.
    #[serde(rename = "History")]
    pub history: Option<History>,

    /// The raw string publication status (e.g., "ppublish").
    #[serde(rename = "PublicationStatus")]
    pub publication_status: String,

    /// Various identifiers linked to the article (DOI, PMC, etc.).
    #[serde(rename = "ArticleIdList")]
    pub article_id_list: ArticleIdList,

    /// List of complex PubMed-specific objects associated with the data.
    #[serde(rename = "ObjectList")]
    pub object_list: Option<ObjectList>,

    /// Bibliographic references cited by the article.
    #[serde(rename = "ReferenceList", default)]
    pub reference_lists: Vec<ReferenceList>,
}

/// Contains PubMed-specific metadata surrounding a book article.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename = "PubmedBookData")]
pub struct PubmedBookData {
    /// Historical dates associated with the publication lifecycle.
    #[serde(rename = "History")]
    pub history: Option<History>,

    /// The raw string publication status.
    #[serde(rename = "PublicationStatus")]
    pub publication_status: String,

    /// Various identifiers linked to the book data.
    #[serde(rename = "ArticleIdList")]
    pub article_id_list: ArticleIdList,

    /// List of complex PubMed-specific objects associated with the book data.
    #[serde(rename = "ObjectList")]
    pub object_list: Option<ObjectList>,
}

/// Holds specific details regarding the published article itself.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename = "Article")]
pub struct Article {
    /// The physical/electronic medium model of publication.
    #[serde(rename = "@PubModel")]
    pub pub_model: ArticlePubModel,

    /// Information about the journal issue.
    #[serde(rename = "Journal")]
    pub journal: Journal,

    /// The title of the article.
    #[serde(rename = "ArticleTitle")]
    pub article_title: Text,

    /// The page range or number for the article.
    /// One of (Pagination, ELocationID*) or (ELocationID+).
    /// Flattened so pagination is optional, and `e_location_ids` can be empty
    /// only when pagination is present.
    #[serde(rename = "Pagination")]
    pub pagination: Option<Pagination>,

    /// Electronic location identifiers.
    #[serde(rename = "ELocationID", default)]
    pub e_location_ids: Vec<ELocationID>,

    /// The article's abstract.
    #[serde(rename = "Abstract")]
    pub abstract_: Option<Abstract>,

    /// The list of authors who wrote the article.
    #[serde(rename = "AuthorList")]
    pub author_list: Option<AuthorList>,

    /// The languages the article is available in (1 or more).
    #[serde(rename = "Language")]
    pub languages: Vec<String>,

    /// External databanks containing associated datasets.
    #[serde(rename = "DataBankList")]
    pub data_bank_list: Option<DataBankList>,

    /// Research grants that funded the work.
    #[serde(rename = "GrantList")]
    pub grant_list: Option<GrantList>,

    /// The types of publication this article represents (e.g., Review,
    /// Clinical Trial).
    #[serde(rename = "PublicationTypeList")]
    pub publication_type_list: PublicationTypeList,

    /// The title of the article in its original language.
    #[serde(rename = "VernacularTitle")]
    pub vernacular_title: Option<Text>,

    /// Important dates in the article's history (e.g., electronic publication
    /// date).
    #[serde(rename = "ArticleDate", default)]
    pub article_dates: Vec<ArticleDate>,
}

// ================================================================
// Alphabetical elements
// ================================================================

/// Represents a standard text node that might contain inline HTML or MathML.
/// Deserialization automatically converts inline content into Markdown.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Text {
    /// The formatted Markdown content.
    #[serde(
        rename = "$value",
        deserialize_with = "deserialize_to_markdown",
        default
    )]
    pub content: String,
}

/// Represents an abstract for an article or document.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename = "Abstract")]
pub struct Abstract {
    /// The text segments of the abstract (1 or more).
    #[serde(rename = "AbstractText")]
    pub texts: Vec<AbstractText>,

    /// Copyright information associated with the abstract.
    #[serde(rename = "CopyrightInformation")]
    pub copyright: Option<String>,
}

/// Represents a single section or paragraph of an abstract text block.
/// Deserialization automatically converts inline content into Markdown.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename = "AbstractText")]
pub struct AbstractText {
    /// The section label (e.g., "METHODS", "RESULTS").
    #[serde(rename = "@Label")]
    pub label: Option<String>,

    /// The structured NLM category for this text segment.
    #[serde(rename = "@NlmCategory")]
    pub nlm_category: Option<NlmCategory>,

    /// The Markdown-formatted content.
    #[serde(
        rename = "$value",
        deserialize_with = "deserialize_to_markdown",
        default
    )]
    pub content: String,
}

/// A wrapper for a list of dataset accession numbers.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename = "AccessionNumberList")]
pub struct AccessionNumberList {
    /// The accession numbers (1 or more).
    #[serde(rename = "AccessionNumber")]
    pub numbers: Vec<String>,
}

/// Contains information about an author's or investigator's institutional
/// affiliation.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename = "AffiliationInfo")]
pub struct AffiliationInfo {
    /// The textual affiliation string.
    #[serde(rename = "Affiliation")]
    pub affiliation: Text,

    /// External identifiers linked to the affiliation (e.g., ROR ID).
    #[serde(rename = "Identifier", default)]
    pub identifiers: Vec<Identifier>,
}

/// Represents a specific date in an article's history, typically its
/// electronic release.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename = "ArticleDate")]
pub struct ArticleDate {
    /// The type of date, always "Electronic" per the DTD FIXED attribute.
    #[serde(rename = "@DateType", default = "fixed_electronic")]
    pub date_type: String,

    /// The year (YYYY).
    #[serde(rename = "Year")]
    pub year: String,

    /// The month (MM or name).
    #[serde(rename = "Month")]
    pub month: String,

    /// The day (DD).
    #[serde(rename = "Day")]
    pub day: String,
}

/// Default date fallback function.
fn fixed_electronic() -> String {
    "Electronic".to_string()
}

/// Represents an external identifier associated with an article.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename = "ArticleId")]
pub struct ArticleId {
    /// The specific type of the ID (e.g., DOI, PMC).
    #[serde(rename = "@IdType", default)]
    pub id_type: ArticleIdType,

    /// The identifier value.
    #[serde(rename = "$text", default)]
    pub value: String,
}

/// A wrapper list for various article identifiers.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename = "ArticleIdList")]
pub struct ArticleIdList {
    /// The list of identifiers (1 or more).
    #[serde(rename = "ArticleId")]
    pub ids: Vec<ArticleId>,
}

/// Represents the name of a contributing individual or group.
/// Can be either a segmented personal name or a collective organization name.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum AuthorName {
    /// A structured personal name.
    Personal {
        /// The person's last name or surname.
        last_name: String,
        /// The person's given or first name.
        fore_name: Option<String>,
        /// The person's initials.
        initials: Option<String>,
        /// Generational suffix (e.g., Jr., III).
        suffix: Option<String>,
    },
    /// The name of an organization or group acting collectively.
    Collective(String),
}

/// Represents a single author who contributed to a document.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename = "Author")]
pub struct Author {
    /// Indicates if the author name is structurally valid.
    #[serde(rename = "@ValidYN", default)]
    pub valid_yn: YN,

    /// Indicates if this author contributed equally with others.
    #[serde(rename = "@EqualContrib")]
    pub equal_contrib: Option<YN>,

    /// Author's surname.
    #[serde(rename = "LastName")]
    pub last_name: Option<String>,

    /// Author's given name.
    #[serde(rename = "ForeName")]
    pub fore_name: Option<String>,

    /// Author's initials.
    #[serde(rename = "Initials")]
    pub initials: Option<String>,

    /// Author's generational suffix.
    #[serde(rename = "Suffix")]
    pub suffix: Option<String>,

    /// Group or organizational name.
    #[serde(rename = "CollectiveName")]
    pub collective_name: Option<String>,

    /// Author identifiers (e.g., ORCID).
    #[serde(rename = "Identifier", default)]
    pub identifiers: Vec<Identifier>,

    /// Institutional affiliations.
    #[serde(rename = "AffiliationInfo", default)]
    pub affiliation_infos: Vec<AffiliationInfo>,
}

/// Wraps a collection of authors for a specific document.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename = "AuthorList")]
pub struct AuthorList {
    /// Indicates if the list is complete.
    #[serde(rename = "@CompleteYN", default)]
    pub complete_yn: YN,

    /// Differentiates between authors and editors.
    #[serde(rename = "@Type")]
    pub list_type: Option<AuthorListType>,

    /// The authors in the list (1 or more).
    #[serde(rename = "Author")]
    pub authors: Vec<Author>,
}

/// A flexible date structure accommodating optional Month/Day or Season
/// fields.
/// Frequently used for `BeginningDate`, `EndingDate`, and `ContributionDate`.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DateFlex {
    /// The year.
    #[serde(rename = "Year")]
    pub year: String,

    /// The optional month.
    #[serde(rename = "Month")]
    pub month: Option<String>,

    /// The optional day.
    #[serde(rename = "Day")]
    pub day: Option<String>,

    /// An optional seasonal descriptor (e.g., "Fall").
    #[serde(rename = "Season")]
    pub season: Option<String>,
}

/// Contains information specific to a book entity.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename = "Book")]
pub struct Book {
    /// The publishing organization.
    #[serde(rename = "Publisher")]
    pub publisher: Publisher,

    /// The title of the book.
    #[serde(rename = "BookTitle")]
    pub book_title: String,

    /// The publication date.
    #[serde(rename = "PubDate")]
    pub pub_date: PubDate,

    /// The starting date of coverage or creation.
    #[serde(rename = "BeginningDate")]
    pub beginning_date: Option<DateFlex>,

    /// The ending date of coverage or creation.
    #[serde(rename = "EndingDate")]
    pub ending_date: Option<DateFlex>,

    /// Lists of authors contributing to the book.
    #[serde(rename = "AuthorList", default)]
    pub author_lists: Vec<AuthorList>,

    /// Investigators associated with the book.
    #[serde(rename = "InvestigatorList")]
    pub investigator_list: Option<InvestigatorList>,

    /// The volume number.
    #[serde(rename = "Volume")]
    pub volume: Option<String>,

    /// The title of the specific volume.
    #[serde(rename = "VolumeTitle")]
    pub volume_title: Option<String>,

    /// The edition of the book.
    #[serde(rename = "Edition")]
    pub edition: Option<String>,

    /// The title of the larger collection this book belongs to.
    #[serde(rename = "CollectionTitle")]
    pub collection_title: Option<String>,

    /// Associated ISBN numbers.
    #[serde(rename = "Isbn", default)]
    pub isbns: Vec<String>,

    /// Electronic location identifiers.
    #[serde(rename = "ELocationID", default)]
    pub e_location_ids: Vec<ELocationID>,

    /// The publication medium.
    #[serde(rename = "Medium")]
    pub medium: Option<String>,

    /// Technical or institutional report number.
    #[serde(rename = "ReportNumber")]
    pub report_number: Option<String>,
}

/// Represents a chemical substance discussed in a citation.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename = "Chemical")]
pub struct Chemical {
    /// The unique chemical registry number.
    #[serde(rename = "RegistryNumber")]
    pub registry_number: String,

    /// The name of the substance.
    #[serde(rename = "NameOfSubstance")]
    pub name_of_substance: NameOfSubstance,
}

/// A wrapper for a list of chemicals.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename = "ChemicalList")]
pub struct ChemicalList {
    /// The chemical substances (1 or more).
    #[serde(rename = "Chemical")]
    pub chemicals: Vec<Chemical>,
}

/// Describes a link between the current document and another publication
/// (e.g., errata, retractions, comments).
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename = "CommentsCorrections")]
pub struct CommentsCorrections {
    /// The specific type of relationship.
    #[serde(rename = "@RefType")]
    pub ref_type: CommentsCorrectionsRefType,

    /// A text string detailing the reference source.
    #[serde(rename = "RefSource")]
    pub ref_source: String,

    /// The PMID of the linked publication, if available.
    #[serde(rename = "PMID")]
    pub pmid: Option<Pmid>,

    /// An optional contextual note.
    #[serde(rename = "Note")]
    pub note: Option<String>,
}

/// Wraps a collection of related comment and correction references.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename = "CommentsCorrectionsList")]
pub struct CommentsCorrectionsList {
    /// The references (1 or more).
    #[serde(rename = "CommentsCorrections")]
    pub items: Vec<CommentsCorrections>,
}

/// Represents an external databank containing supplementary datasets.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename = "DataBank")]
pub struct DataBank {
    /// The name of the databank (e.g., PDB, ClinicalTrials.gov).
    #[serde(rename = "DataBankName")]
    pub name: String,

    /// The specific accession numbers associated with the dataset.
    #[serde(rename = "AccessionNumberList")]
    pub accession_number_list: Option<AccessionNumberList>,
}

/// A wrapper for a list of databanks.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename = "DataBankList")]
pub struct DataBankList {
    /// Indicates if the list is complete.
    #[serde(rename = "@CompleteYN", default)]
    pub complete_yn: YN,

    /// The databanks (1 or more).
    #[serde(rename = "DataBank")]
    pub data_banks: Vec<DataBank>,
}

/// A simple structure for dates containing exactly Year, Month, and Day.
/// Used for elements like `DateCompleted`, `DateRevised`, and
/// `ContributionDate`.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DateYMD {
    /// The year.
    #[serde(rename = "Year")]
    pub year: String,

    /// The month.
    #[serde(rename = "Month")]
    pub month: String,

    /// The day.
    #[serde(rename = "Day")]
    pub day: String,
}

/// The name component of a Medical Subject Heading (MeSH) descriptor.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename = "DescriptorName")]
pub struct DescriptorName {
    /// Indicates if this is a major topic of the article.
    #[serde(rename = "@MajorTopicYN", default)]
    pub major_topic_yn: YN,

    /// Implied attribute, always "Y" if present, denoting an automated
    /// assignment.
    #[serde(rename = "@AutoHM")]
    pub auto_hm: Option<String>,

    /// Denotes if this is a specialized descriptor, like Geographic.
    #[serde(rename = "@Type")]
    pub type_: Option<DescriptorNameType>,

    /// The unique identifier of the descriptor.
    #[serde(rename = "@UI")]
    pub ui: String,

    /// The textual name of the descriptor.
    #[serde(rename = "$text", default)]
    pub value: String,
}

/// Represents an electronic location ID, like a DOI or PII.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename = "ELocationID")]
pub struct ELocationID {
    /// The type of identifier.
    #[serde(rename = "@EIdType")]
    pub eid_type: EIdType,

    /// Indicates if the ID is considered valid.
    #[serde(rename = "@ValidYN", default)]
    pub valid_yn: YN,

    /// The identifier string itself.
    #[serde(rename = "$text", default)]
    pub value: String,
}

/// Wraps a list of gene symbols referenced in the article.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename = "GeneSymbolList")]
pub struct GeneSymbolList {
    /// The gene symbols (1 or more).
    #[serde(rename = "GeneSymbol")]
    pub symbols: Vec<String>,
}

/// A generalized note attached to the document by the indexer or owner.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename = "GeneralNote")]
pub struct GeneralNote {
    /// The organization that added the note.
    #[serde(rename = "@Owner", default)]
    pub owner: OwnerNote,

    /// The text of the note.
    #[serde(rename = "$text", default)]
    pub value: String,
}

/// Represents a research grant that supported the work in the publication.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename = "Grant")]
pub struct Grant {
    /// The unique ID of the grant.
    #[serde(rename = "GrantID")]
    pub grant_id: Option<String>,

    /// The acronym of the funding agency.
    #[serde(rename = "Acronym")]
    pub acronym: Option<String>,

    /// The name of the funding agency.
    #[serde(rename = "Agency")]
    pub agency: String,

    /// The country where the agency is located.
    #[serde(rename = "Country")]
    pub country: Option<String>,
}

/// Wraps a collection of funding grants.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename = "GrantList")]
pub struct GrantList {
    /// Indicates if the list is complete.
    #[serde(rename = "@CompleteYN", default)]
    pub complete_yn: YN,

    /// The list of grants (1 or more).
    #[serde(rename = "Grant")]
    pub grants: Vec<Grant>,
}

/// Represents the historical timeline of the document's presence in PubMed.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename = "History")]
pub struct History {
    /// The specific historical dates (1 or more).
    #[serde(rename = "PubMedPubDate")]
    pub pub_dates: Vec<PubMedPubDate>,
}

/// An abstract identifier string associated with an external source.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename = "Identifier")]
pub struct Identifier {
    /// The source or namespace of the identifier (e.g., ORCID).
    #[serde(rename = "@Source")]
    pub source: String,

    /// The identifier value.
    #[serde(rename = "$text", default)]
    pub value: String,
}

/// Represents an investigator or collaborator who is not classified as and
/// primary author.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename = "Investigator")]
pub struct Investigator {
    /// Indicates if the name is structurally valid.
    #[serde(rename = "@ValidYN", default)]
    pub valid_yn: YN,

    /// The investigator's surname.
    #[serde(rename = "LastName")]
    pub last_name: String,

    /// The investigator's given name.
    #[serde(rename = "ForeName")]
    pub fore_name: Option<String>,

    /// The investigator's initials.
    #[serde(rename = "Initials")]
    pub initials: Option<String>,

    /// The investigator's generational suffix.
    #[serde(rename = "Suffix")]
    pub suffix: Option<String>,

    /// External identifiers for the investigator.
    #[serde(rename = "Identifier", default)]
    pub identifiers: Vec<Identifier>,

    /// Institutional affiliations.
    #[serde(rename = "AffiliationInfo", default)]
    pub affiliation_infos: Vec<AffiliationInfo>,
}

/// Wraps a collection of investigators.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename = "InvestigatorList")]
pub struct InvestigatorList {
    /// Optional identifier for the specific list of investigators.
    #[serde(rename = "@ID")]
    pub id: Option<String>,

    /// The investigators (1 or more).
    #[serde(rename = "Investigator")]
    pub investigators: Vec<Investigator>,
}

/// Represents an International Standard Serial Number (ISSN).
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename = "ISSN")]
pub struct Issn {
    /// The medium type of the ISSN (Print or Electronic).
    #[serde(rename = "@IssnType")]
    pub issn_type: IssnType,

    /// The ISSN string.
    #[serde(rename = "$text", default)]
    pub value: String,
}

/// A generic list of categorized text items.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename = "ItemList")]
pub struct ItemList {
    /// The category or type of the list.
    #[serde(rename = "@ListType")]
    pub list_type: String,

    /// The textual items (1 or more).
    #[serde(rename = "Item")]
    pub items: Vec<String>,
}

/// Contains identifying information for the journal publishing the article.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename = "Journal")]
pub struct Journal {
    /// The journal's ISSN.
    #[serde(rename = "ISSN")]
    pub issn: Option<Issn>,

    /// Details of the specific journal issue.
    #[serde(rename = "JournalIssue")]
    pub journal_issue: JournalIssue,

    /// The full title of the journal.
    #[serde(rename = "Title")]
    pub title: Option<String>,

    /// The standardized ISO abbreviation for the journal title.
    #[serde(rename = "ISOAbbreviation")]
    pub iso_abbreviation: Option<String>,
}

/// Contains information regarding a specific publication issue of a journal.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename = "JournalIssue")]
pub struct JournalIssue {
    /// The medium of the issue (Print or Internet).
    #[serde(rename = "@CitedMedium")]
    pub cited_medium: CitedMedium,

    /// The volume number.
    #[serde(rename = "Volume")]
    pub volume: Option<String>,

    /// The issue number.
    #[serde(rename = "Issue")]
    pub issue: Option<String>,

    /// The date of publication.
    #[serde(rename = "PubDate")]
    pub pub_date: PubDate,
}

/// Represents a single indexing keyword. Mixed content converts to Markdown.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename = "Keyword")]
pub struct Keyword {
    /// Indicates if the keyword is a major topic.
    #[serde(rename = "@MajorTopicYN", default)]
    pub major_topic_yn: YN,

    /// The Markdown-formatted text of the keyword.
    #[serde(
        rename = "$value",
        deserialize_with = "deserialize_to_markdown",
        default
    )]
    pub content: String,
}

/// Wraps a collection of indexing keywords.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename = "KeywordList")]
pub struct KeywordList {
    /// The organization providing the keyword list.
    #[serde(rename = "@Owner", default)]
    pub owner: KeywordListOwner,

    /// The keywords (1 or more).
    #[serde(rename = "Keyword")]
    pub keywords: Vec<Keyword>,
}

/// Provides a structural label for locations within a document (e.g.,
/// 'Chapter 5').
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename = "LocationLabel")]
pub struct LocationLabel {
    /// The structural classification (e.g., Chapter, Section).
    #[serde(rename = "@Type")]
    pub type_: Option<LocationLabelType>,

    /// The text of the label.
    #[serde(rename = "$text", default)]
    pub value: String,
}

/// Contains internal NLM cataloging information for the journal.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename = "MedlineJournalInfo")]
pub struct MedlineJournalInfo {
    /// The country of publication.
    #[serde(rename = "Country")]
    pub country: Option<String>,

    /// The MEDLINE title abbreviation.
    #[serde(rename = "MedlineTA")]
    pub medline_ta: String,

    /// The NLM-specific unique identifier for the journal.
    #[serde(rename = "NlmUniqueID")]
    pub nlm_unique_id: Option<String>,

    /// The primary ISSN linking the journal.
    #[serde(rename = "ISSNLinking")]
    pub issn_linking: Option<String>,
}

/// Represents a single Medical Subject Heading (MeSH) applied to the
/// document.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename = "MeshHeading")]
pub struct MeshHeading {
    /// The primary concept name.
    #[serde(rename = "DescriptorName")]
    pub descriptor_name: DescriptorName,

    /// Optional specific sub-qualifiers for the concept.
    #[serde(rename = "QualifierName", default)]
    pub qualifier_names: Vec<QualifierName>,
}

/// Wraps a collection of MeSH headings.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename = "MeshHeadingList")]
pub struct MeshHeadingList {
    /// The MeSH headings (1 or more).
    #[serde(rename = "MeshHeading")]
    pub headings: Vec<MeshHeading>,
}

/// Represents the name of a chemical substance associated with a MeSH
/// heading.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename = "NameOfSubstance")]
pub struct NameOfSubstance {
    /// The unique identifier.
    #[serde(rename = "@UI")]
    pub ui: String,

    /// The substance name text.
    #[serde(rename = "$text", default)]
    pub value: String,
}

/// A generic PubMed object holding arbitrary parameterized metadata.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename = "Object")]
pub struct PubmedObject {
    /// The structural type of the object.
    #[serde(rename = "@Type")]
    pub type_: String,

    /// Key-value parameters defining the object.
    #[serde(rename = "Param", default)]
    pub params: Vec<Param>,
}

/// A wrapper for a list of abstract PubMed objects.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename = "ObjectList")]
pub struct ObjectList {
    /// The objects (1 or more).
    #[serde(rename = "Object")]
    pub objects: Vec<PubmedObject>,
}

/// Represents an abstract provided by an organization other than NLM.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename = "OtherAbstract")]
pub struct OtherAbstract {
    /// The type or source of the abstract.
    #[serde(rename = "@Type")]
    pub type_: OtherAbstractType,

    /// The language of the abstract, defaulting to English ("eng").
    #[serde(rename = "@Language", default = "default_lang")]
    pub language: String,

    /// The text segments of the abstract (1 or more).
    #[serde(rename = "AbstractText")]
    pub texts: Vec<AbstractText>,

    /// Copyright information for the abstract.
    #[serde(rename = "CopyrightInformation")]
    pub copyright: Option<String>,
}

/// Default abstract language.
fn default_lang() -> String {
    "eng".to_string()
}

/// Represents a tracking identifier assigned by a non-NLM organization.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename = "OtherID")]
pub struct OtherId {
    /// The source organization.
    #[serde(rename = "@Source")]
    pub source: OtherIdSource,

    /// The identifier string.
    #[serde(rename = "$text", default)]
    pub value: String,
}

/// Represents a PubMed Unique Identifier.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename = "PMID")]
pub struct Pmid {
    /// The version of the PMID.
    #[serde(rename = "@Version")]
    pub version: String,

    /// The integer identifier value.
    #[serde(rename = "$text", default)]
    pub value: u32,
}

/// Defines the page ranges for a publication.
///
/// DTD: ((StartPage, EndPage?, MedlinePgn?) | MedlinePgn).
/// Flattened in Rust: all fields are optional, enabling runtime validation.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename = "Pagination")]
pub struct Pagination {
    /// The starting page.
    #[serde(rename = "StartPage")]
    pub start_page: Option<String>,

    /// The ending page.
    #[serde(rename = "EndPage")]
    pub end_page: Option<String>,

    /// A concatenated string representing complex pagination (e.g.,
    /// "12-4, 18").
    #[serde(rename = "MedlinePgn")]
    pub medline_pgn: Option<String>,
}

/// A key-value pair representing an arbitrary parameter.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename = "Param")]
pub struct Param {
    /// The name of the parameter.
    #[serde(rename = "@Name")]
    pub name: String,

    /// The parameter's string value.
    #[serde(rename = "$text", default)]
    pub value: String,
}

/// Represents a specific individual who is a primary subject of the article
/// (e.g., in a biography).
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename = "PersonalNameSubject")]
pub struct PersonalNameSubject {
    /// The person's surname.
    #[serde(rename = "LastName")]
    pub last_name: String,

    /// The person's given name.
    #[serde(rename = "ForeName")]
    pub fore_name: Option<String>,

    /// The person's initials.
    #[serde(rename = "Initials")]
    pub initials: Option<String>,

    /// The person's generational suffix.
    #[serde(rename = "Suffix")]
    pub suffix: Option<String>,
}

/// Wraps a collection of personal name subjects.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename = "PersonalNameSubjectList")]
pub struct PersonalNameSubjectList {
    /// The subjects (1 or more).
    #[serde(rename = "PersonalNameSubject")]
    pub subjects: Vec<PersonalNameSubject>,
}

/// Defines the complex publication date representations found in PubMed.
///
/// DTD: ((Year, ((Month, Day?) | Season)?) | MedlineDate).
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename = "PubDate")]
pub struct PubDate {
    /// The publication year.
    #[serde(rename = "Year")]
    pub year: Option<String>,

    /// The publication month.
    #[serde(rename = "Month")]
    pub month: Option<String>,

    /// The publication day.
    #[serde(rename = "Day")]
    pub day: Option<String>,

    /// A seasonal publication descriptor.
    #[serde(rename = "Season")]
    pub season: Option<String>,

    /// A raw string used for unparseable or date-range text formats.
    #[serde(rename = "MedlineDate")]
    pub medline_date: Option<String>,
}

/// Represents a granular timestamp in the document's PubMed timeline.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename = "PubMedPubDate")]
pub struct PubMedPubDate {
    /// The life-cycle event this timestamp represents.
    #[serde(rename = "@PubStatus")]
    pub pub_status: PubStatus,

    /// The year.
    #[serde(rename = "Year")]
    pub year: String,

    /// The month.
    #[serde(rename = "Month")]
    pub month: String,

    /// The day.
    #[serde(rename = "Day")]
    pub day: String,

    /// The hour (optional).
    #[serde(rename = "Hour")]
    pub hour: Option<String>,

    /// The minute (optional).
    #[serde(rename = "Minute")]
    pub minute: Option<String>,

    /// The second (optional).
    #[serde(rename = "Second")]
    pub second: Option<String>,
}

/// Contains information about the publishing entity.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename = "Publisher")]
pub struct Publisher {
    /// The name of the publisher.
    #[serde(rename = "PublisherName")]
    pub name: String,

    /// The physical location/city of the publisher.
    #[serde(rename = "PublisherLocation")]
    pub location: Option<String>,
}

/// Identifies the semantic type of publication (e.g., "Review", "Editorial").
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename = "PublicationType")]
pub struct PublicationType {
    /// The unique identifier.
    #[serde(rename = "@UI")]
    pub ui: String,

    /// The name of the publication type.
    #[serde(rename = "$text", default)]
    pub value: String,
}

/// Wraps a collection of publication types.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename = "PublicationTypeList")]
pub struct PublicationTypeList {
    /// The publication types (1 or more).
    #[serde(rename = "PublicationType", default)]
    pub types: Vec<PublicationType>,
}

/// Further categorizes a primary `DescriptorName` in a MeSH Heading.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename = "QualifierName")]
pub struct QualifierName {
    /// Implied automated assignment attribute.
    #[serde(rename = "@AutoHM")]
    pub auto_hm: Option<String>,

    /// Indicates if this specific sub-qualifier is a major topic.
    #[serde(rename = "@MajorTopicYN", default)]
    pub major_topic_yn: YN,

    /// The unique identifier.
    #[serde(rename = "@UI")]
    pub ui: String,

    /// The text of the qualifier.
    #[serde(rename = "$text", default)]
    pub value: String,
}

/// Represents a single bibliographic reference cited within the document.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename = "Reference")]
pub struct Reference {
    /// The raw citation text.
    #[serde(rename = "Citation")]
    pub citation: Text,

    /// Any identifiers linking the citation to known external records.
    #[serde(rename = "ArticleIdList")]
    pub article_id_list: Option<ArticleIdList>,
}

/// Wraps a collection of bibliographic references, supporting recursive
/// sub-lists.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename = "ReferenceList")]
pub struct ReferenceList {
    /// The title of the reference list.
    #[serde(rename = "Title")]
    pub title: Option<String>,

    /// The references in this tier.
    #[serde(rename = "Reference", default)]
    pub references: Vec<Reference>,

    /// Nested sub-lists of references.
    #[serde(rename = "ReferenceList", default)]
    pub sub_lists: Vec<ReferenceList>,
}

/// Represents a structural section within a document, allowing for deep
/// nesting.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename = "Section")]
pub struct Section {
    /// The structural location type.
    #[serde(rename = "LocationLabel")]
    pub location_label: Option<LocationLabel>,

    /// The title of the section.
    #[serde(rename = "SectionTitle")]
    pub section_title: String,

    /// Nested sub-sections.
    #[serde(rename = "Section", default)]
    pub sub_sections: Vec<Section>,
}

/// Wraps a collection of structural document sections.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename = "Sections")]
pub struct Sections {
    /// The sections (1 or more).
    #[serde(rename = "Section")]
    pub sections: Vec<Section>,
}

/// Represents a supplementary MeSH concept (like a specific rare disease or
/// chemical).
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename = "SupplMeshName")]
pub struct SupplMeshName {
    /// The specific category of the concept.
    #[serde(rename = "@Type")]
    pub type_: SupplMeshNameType,

    /// The unique identifier.
    #[serde(rename = "@UI")]
    pub ui: String,

    /// The name of the concept.
    #[serde(rename = "$text", default)]
    pub value: String,
}

/// Wraps a collection of supplementary MeSH concepts.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename = "SupplMeshList")]
pub struct SupplMeshList {
    /// The concept names (1 or more).
    #[serde(rename = "SupplMeshName")]
    pub names: Vec<SupplMeshName>,
}

