//! Streaming deserialization for PubMed article sets.
//!
//! This module provides a "dumb" deserializer for [`PubmedArticleSet`].
//! Instead of loading thousands of articles into a [`Vec<PubmedArticle>`] and
//! consuming significant memory, this implementation uses the
//! [`DeserializeSeed`] pattern to process each article via a callback as soon
//! as it is parsed.
//!
//! # Benefits
//! * **Memory Efficiency**: Constant memory usage regardless of the number of
//!   articles.
//! * **Early Processing**: You can start processing, saving, or filtering
//!   articles before the entire document is finished parsing.
//!
//! # Example
//! ```rust,ignore
//! let mut count = 0;
//! let seed = PubmedArticleSetSeed {
//!     processor: &|article: PubmedArticle| {
//!         count += 1;
//!         println!("Found article: {}", article.medline_citation.pmid.value);
//!     },
//! };
//! seed.deserialize(deserializer).unwrap();
//! ```

use serde::de::{self, DeserializeSeed, Deserializer, SeqAccess, Visitor};
use std::fmt;

use super::PubmedArticle;

/// A [`DeserializeSeed`] that parses a `<PubmedArticleSet>` and forwards each
/// `<PubmedArticle>` to `processor` without ever collecting them into a
/// [`Vec`].
pub(crate) struct PubmedArticleSetSeed<'a, F>
where
    F: Fn(PubmedArticle) + 'a,
{
    /// The callback invoked for every article found in the XML stream.
    pub(crate) processor: &'a F,
}

impl<'de, 'a, F> DeserializeSeed<'de> for PubmedArticleSetSeed<'a, F>
where
    F: Fn(PubmedArticle) + 'a,
{
    type Value = ();

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_struct(
            "PubmedArticleSet",
            &["PubmedArticle"],
            ArticleSetVisitor {
                processor: self.processor,
            },
        )
    }
}

// ============================================================================
// Visitors
// ============================================================================

/// Visits the top-level `PubmedArticleSet` map and dispatches `PubmedArticle`
/// sequences to [`ArticleSeqSeed`].
struct ArticleSetVisitor<'a, F>
where
    F: Fn(PubmedArticle) + 'a,
{
    /// The callback invoked for every article found in the XML stream.
    processor: &'a F,
}

impl<'de, 'a, F> Visitor<'de> for ArticleSetVisitor<'a, F>
where
    F: Fn(PubmedArticle) + 'a,
{
    type Value = ();

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a PubmedArticleSet")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: de::MapAccess<'de>,
    {
        while let Some(key) = map.next_key::<String>()? {
            if key == "PubmedArticle" {
                map.next_value_seed(ArticleSeqSeed {
                    processor: self.processor,
                })?;
            } else {
                let _: de::IgnoredAny = map.next_value()?;
            }
        }
        Ok(())
    }
}

/// A [`DeserializeSeed`] for the inner sequence of `<PubmedArticle>`
/// elements.
struct ArticleSeqSeed<'a, F>
where
    F: Fn(PubmedArticle) + 'a,
{
    /// The callback invoked for every article found in the XML stream.
    processor: &'a F,
}

impl<'de, 'a, F> DeserializeSeed<'de> for ArticleSeqSeed<'a, F>
where
    F: Fn(PubmedArticle) + 'a,
{
    type Value = ();

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_seq(ArticleSeqVisitor {
            processor: self.processor,
        })
    }
}

/// Visits each element of the article sequence and invokes the callback.
struct ArticleSeqVisitor<'a, F>
where
    F: Fn(PubmedArticle) + 'a,
{
    /// The callback invoked for every article found in the XML stream.
    processor: &'a F,
}

impl<'de, 'a, F> Visitor<'de> for ArticleSeqVisitor<'a, F>
where
    F: Fn(PubmedArticle) + 'a,
{
    type Value = ();

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a sequence of PubmedArticle")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        while let Some(article) = seq.next_element::<PubmedArticle>()? {
            (self.processor)(article);
        }
        Ok(())
    }
}
