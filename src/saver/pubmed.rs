//! Module of a SaverPubMed to CSV

use std::{collections::HashSet, fs::File, path::Path};

use csv::Writer;
use pubmed::chunks::models::{
    ArticleIdType, Author, AuthorType, DateYMD, Identifier, Journal, Keyword,
    KeywordListOwner, MedlineJournalInfo, MeshHeading, PubmedArticle,
    QualifierName, YN,
};

use crate::writer;

/// Struct that regroup CSV Files
pub struct SaverPubMed {
    /// CSV Writer for Article Nodes
    articles: Writer<File>,

    /// CSV Writer for Person Nodes
    persons: Writer<File>,
    /// Set of the ID of saved person node
    person_id: HashSet<String>,

    /// CSV Writer for Collective Nodes
    collectives: Writer<File>,
    /// Set of the ID of saved collective node
    collective_id: HashSet<String>,

    /// CSV Writer for Journal Nodes
    journals: Writer<File>,
    /// Set of the ID of saved journal node
    journal_id: HashSet<String>,

    /// CSV Writer for Keyword Nodes
    keywords: Writer<File>,
    /// Set of the ID of saved keyword node
    keyword_id: HashSet<String>,

    /// CSV Writer for MeSHQualified Nodes
    mesh_qualifieds: Writer<File>,
    /// Set of the ID of saved MeSHQualified node
    qualified_id: HashSet<String>,

    /// CSV Writer for HAS_AUTHOR Relation
    has_author: Writer<File>,

    /// CSV writer for IS_PART_OF Relation
    is_part_of: Writer<File>,

    /// CSV writer for HAS_KEYWORD Relation
    has_keyword: Writer<File>,

    /// CSV writer for CITES Relation
    cites: Writer<File>,

    /// CSV Writer for HAS_MESH Relation
    has_mesh: Writer<File>,

    /// CSV Writer for HAS_DESCRIPTOR Relation
    has_descriptor: Writer<File>,

    /// CSV Writer for HAS_QUALIFIER Relation
    has_qualifier: Writer<File>,
}

impl SaverPubMed {
    /// Init a saver which creates CSV file and writes header
    pub fn new(dir: &Path) -> std::io::Result<Self> {
        Ok(Self {
            articles: writer!(
                dir,
                "Article",
                [
                    "pmid:ID(Article){id-type: long}",
                    "title",
                    "abstract",
                    "dateCompleted:DATE",
                    "dateRevised:DATE",
                ]
            ),
            persons: writer!(
                dir,
                "Person",
                [
                    ":ID(Agent)",
                    "lastName",
                    "foreName",
                    "initials",
                    "suffix",
                    "orcid",
                ]
            ),
            person_id: HashSet::new(),
            collectives: writer!(dir, "Collective", ["name:ID(Agent)",]),
            collective_id: HashSet::new(),
            journals: writer!(
                dir,
                "Journal",
                [
                    ":ID(Journal)",
                    "title",
                    "country",
                    "nlmId",
                    "issn",
                    "isoAbbreviation",
                ]
            ),
            journal_id: HashSet::new(),
            keywords: writer!(
                dir,
                "Keyword",
                [":ID(Keyword)", "value", "supplier",]
            ),
            keyword_id: HashSet::new(),
            mesh_qualifieds: writer!(
                dir,
                "MeSHQualified",
                [":ID(MeSHQualified)"]
            ),
            qualified_id: HashSet::new(),
            has_author: writer!(
                dir,
                "HAS_AUTHOR",
                [":START_ID(Article)", ":END_ID(Agent)",]
            ),
            is_part_of: writer!(
                dir,
                "IS_PART_OF",
                [":START_ID(Article)", ":END_ID(Journal)",]
            ),
            has_keyword: writer!(
                dir,
                "HAS_KEYWORD",
                [":START_ID(Article)", ":END_ID(Keyword)",]
            ),
            cites: writer!(
                dir,
                "CITES",
                [":START_ID(Article)", ":END_ID(Article)",]
            ),
            has_mesh: writer!(
                dir,
                "HAS_MESH",
                [":START_ID(Article)", ":END_ID(MeSHQualified)",]
            ),
            has_descriptor: writer!(
                dir,
                "HAS_DESCRIPTOR",
                [
                    ":START_ID(MeSHQualified)",
                    "majorTopic:boolean",
                    ":END_ID(MeSH)",
                ]
            ),
            has_qualifier: writer!(
                dir,
                "HAS_QUALIFIER",
                [
                    ":START_ID(MeSHQualified)",
                    "majorTopic:boolean",
                    ":END_ID(MeSH)",
                ]
            ),
        })
    }

    /// Flush every CSV file
    pub fn flush(&mut self) -> std::io::Result<()> {
        self.articles.flush()?;
        self.persons.flush()?;
        self.collectives.flush()?;
        self.journals.flush()?;
        self.keywords.flush()?;
        self.mesh_qualifieds.flush()?;
        self.has_author.flush()?;
        self.is_part_of.flush()?;
        self.has_keyword.flush()?;
        self.cites.flush()?;
        self.has_mesh.flush()?;
        self.has_descriptor.flush()?;
        self.has_qualifier.flush()?;

        Ok(())
    }

    /// Save one MeSH
    pub fn add_mesh(
        &mut self,
        mesh: &MeshHeading,
        pmid: &str,
    ) -> std::io::Result<()> {
        let mut quals: Vec<&str> = mesh
            .qualifier_names
            .iter()
            .map(|q: &QualifierName| q.ui.as_str())
            .collect();
        quals.sort();
        let quals: String = quals.join(":");
        let mesh_id: String = format!("{}-{}", mesh.descriptor_name.ui, quals);

        self.has_mesh.write_record([pmid, &mesh_id])?;

        if !self.qualified_id.contains(&mesh_id) {
            self.mesh_qualifieds.write_record([&mesh_id])?;

            self.has_descriptor.write_record([
                &mesh_id,
                matches!(mesh.descriptor_name.major_topic_yn, YN::Y)
                    .to_string()
                    .as_str(),
                mesh.descriptor_name.ui.as_str(),
            ])?;

            for qual in mesh.qualifier_names.iter() {
                self.has_qualifier.write_record([
                    &mesh_id,
                    matches!(qual.major_topic_yn, YN::Y).to_string().as_str(),
                    qual.ui.as_str(),
                ])?;
            }

            self.qualified_id.insert(mesh_id);
        }

        Ok(())
    }

    /// Save one citation
    pub fn add_cites(&mut self, source: &str, to: &str) -> std::io::Result<()> {
        self.cites.write_record([source, to])?;
        Ok(())
    }

    /// Save one keyword
    pub fn add_keyword(
        &mut self,
        keyword: &Keyword,
        owner: &KeywordListOwner,
        pmid: &str,
    ) -> std::io::Result<()> {
        let owner: String = format!("{owner:?}");
        let keyword_id: String = format!("{owner}-{}", keyword.content);

        self.has_keyword.write_record([pmid, &keyword_id])?;

        if !self.keyword_id.contains(&keyword_id) {
            self.keywords.write_record([
                &keyword_id,
                keyword.content.as_str(),
                &owner,
            ])?;

            self.keyword_id.insert(keyword_id);
        }
        Ok(())
    }

    /// Save one author
    pub fn add_author(
        &mut self,
        author: &Author,
        pmid: &str,
    ) -> std::io::Result<()> {
        match &author.name {
            AuthorType::Person {
                last_name,
                fore_name,
                initials,
                suffix,
            } => {
                let fore_name: &str = fore_name.as_deref().unwrap_or("");
                let suffix: &str = suffix.as_deref().unwrap_or("");
                let orcid: Option<&str> =
                    author.identifiers.iter().find_map(|id| match id {
                        Identifier::Orcid(id) if !id.is_empty() => {
                            Some(id.as_str())
                        }
                        _ => None,
                    });
                let person_id: String = match orcid {
                    Some(id) => format!("ORCID:{id}"),
                    None => format!("{}-{}-{}", fore_name, last_name, suffix),
                };

                self.has_author.write_record([pmid, &person_id])?;

                if !self.person_id.contains(&person_id) {
                    self.persons.write_record([
                        &person_id,
                        last_name,
                        fore_name,
                        initials.as_deref().unwrap_or(""),
                        suffix,
                        orcid.unwrap_or(""),
                    ])?;

                    self.person_id.insert(person_id);
                }
            }
            AuthorType::Collective { name } => {
                if self.collective_id.insert(name.clone()) {
                    self.collectives.write_record([name])?;
                }
                self.has_author.write_record([pmid, name])?;
            }
        }

        Ok(())
    }

    /// Save one journal
    pub fn add_journal(
        &mut self,
        journal: &Journal,
        journal_info: &MedlineJournalInfo,
        pmid: &str,
    ) -> std::io::Result<()> {
        let journal_id: String = match &journal_info.nlm_unique_id {
            Some(nlm) => format!("NLM:{}", nlm),
            None => {
                format!("TA:{}", journal_info.medline_ta)
            }
        };

        self.is_part_of.write_record([pmid, &journal_id])?;

        if !self.journal_id.contains(&journal_id) {
            self.journals.write_record([
                &journal_id,
                journal.title.as_deref().unwrap_or(""),
                journal_info.country.as_deref().unwrap_or(""),
                journal_info.nlm_unique_id.as_deref().unwrap_or(""),
                journal
                    .issn
                    .as_ref()
                    .map(|i| i.value.as_str())
                    .unwrap_or(""),
                journal.iso_abbreviation.as_deref().unwrap_or(""),
            ])?;

            self.journal_id.insert(journal_id);
        }

        Ok(())
    }

    /// Save one article
    pub fn add_article(
        &mut self,
        article: &PubmedArticle,
    ) -> std::io::Result<()> {
        let pmid: String = article.medline_citation.pmid.value.to_string();

        let abstract_text: String = article
            .medline_citation
            .article
            .abstract_
            .as_ref()
            .map(|a| {
                a.texts
                    .iter()
                    .map(|t| t.content.as_str())
                    .collect::<Vec<&str>>()
                    .join("\n\n")
            })
            .unwrap_or_default();

        self.articles.write_record([
            &pmid,
            &article.medline_citation.article.title.content,
            &abstract_text,
            &date_to_string(article.medline_citation.date_completed.as_ref()),
            &date_to_string(article.medline_citation.date_revised.as_ref()),
        ])?;

        if let Some(authors) = &article.medline_citation.article.author_list {
            for author in authors.authors.iter() {
                self.add_author(author, &pmid)?;
            }
        }

        for keywords in article.medline_citation.keyword_lists.iter() {
            for keyword in keywords.keywords.iter() {
                self.add_keyword(keyword, &keywords.owner, &pmid)?;
            }
        }

        if let Some(meshs) = &article.medline_citation.mesh_heading_list {
            for mesh in meshs.headings.iter() {
                self.add_mesh(mesh, &pmid)?;
            }
        }

        if let Some(pubmed) = &article.pubmed_data {
            for cites_list in pubmed.reference_lists.iter() {
                for cites in cites_list.references.iter() {
                    if let Some(cite) =
                        cites.article_id_list.as_ref().and_then(|l| {
                            l.ids.iter().find(|id| {
                                matches!(id.id_type, ArticleIdType::PUBMED)
                                    && id.value.parse::<u32>().is_ok()
                            })
                        })
                    {
                        self.add_cites(&pmid, &cite.value)?;
                    }
                }
            }
        }

        self.add_journal(
            &article.medline_citation.article.journal,
            &article.medline_citation.medline_journal_info,
            &pmid,
        )?;

        Ok(())
    }
}

/// Converts an optional date reference into a formatted ISO 8601 string.
///
/// This function takes an `Option<&DateYMD>` and returns a string in the
/// format `YYYY-MM-DD`. If the input is `None`, it returns an empty string.
fn date_to_string(d: Option<&DateYMD>) -> String {
    d.map(|d| format!("{:04}-{:02}-{:02}", d.year, d.month, d.day))
        .unwrap_or_default()
}
