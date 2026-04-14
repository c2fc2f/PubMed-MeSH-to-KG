//! Module of a Saver to CSV

use std::{collections::HashSet, fs::File, path::Path};

use csv::Writer;
use pubmed::chunks::models::{
    Author, AuthorType, DateYMD, Identifier, Journal, MedlineJournalInfo,
    PubmedArticle,
};

/// Macro to initialize CSV Writer
macro_rules! writer {
    ($dir:expr, $name:literal, [$($field:literal),* $(,)?]) => {{
        let mut w = Writer::from_writer(
            File::create($dir.join(concat!($name, ".csv")))?
        );
        w.write_record([$($field),*])?;
        w
    }};
}

/// Struct that regroup CSV Files
pub struct Saver {
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

    /// CSV Writer for hasAuthor Relation for Person Node
    has_author_person: Writer<File>,
    /// CSV Writer for hasAuthor Relation for Collective Node
    has_author_collective: Writer<File>,

    /// CSV writer for isPartOf Relation
    is_part_of: Writer<File>,
}

impl Saver {
    /// Init a saver which creates CSV file and writes header
    pub fn new(dir: &Path) -> std::io::Result<Self> {
        Ok(Self {
            articles: writer!(
                dir,
                "Article",
                [
                    ":ID(Article)",
                    "pmid:int",
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
                    ":ID(Person)",
                    "lastName",
                    "foreName",
                    "initials",
                    "suffix",
                    "orcid",
                ]
            ),
            person_id: HashSet::new(),
            collectives: writer!(dir, "Collective", ["name:ID(Collective)",]),
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
            has_author_person: writer!(
                dir,
                "hasAuthor-Person",
                [":START_ID(Article)", ":END_ID(Person)",]
            ),
            has_author_collective: writer!(
                dir,
                "hasAuthor-Collective",
                [":START_ID(Article)", ":END_ID(Collective)",]
            ),
            is_part_of: writer!(
                dir,
                "isPartOf",
                [":START_ID(Article)", ":END_ID(Journal)",]
            ),
        })
    }

    /// Flush every CSV file
    pub fn flush(&mut self) -> std::io::Result<()> {
        self.articles.flush()?;
        self.persons.flush()?;
        self.collectives.flush()?;
        self.journals.flush()?;
        self.has_author_person.flush()?;
        self.has_author_collective.flush()?;
        self.is_part_of.flush()?;

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
                    Some(id) => id.to_string(),
                    None => format!("{}-{}-{}", fore_name, last_name, suffix),
                };

                self.has_author_person.write_record([pmid, &person_id])?;

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
                self.has_author_collective.write_record([pmid, name])?;
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
