//! AST and deserialization logic for MathML and inline HTML to Markdown
//! conversion.
//!
//! This module defines the `ContentNode` enum, which represents various
//! supported HTML formatting tags (such as `<b>`, `<i>`, `<sub>`) and MathML
//! structural elements (such as `<math>`, `<mfrac>`, `<msub>`).
//!
//! It provides functionality to deserialize these structures from
//! XML/HTML-like data streams using [`serde`], and includes a recursive
//! engine to convert the resulting Abstract Syntax Tree (AST) into a clean,
//! LaTeX-flavored Markdown string representation.

use serde::{Deserialize, Deserializer};

/// Represents an abstract syntax tree (AST) node for MathML and inline HTML
/// content. This enum is used to deserialize structured XML/HTML data and
/// convert it into LaTeX-flavored Markdown representation.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
enum ContentNode {
    /// A raw text node containing character data without any enclosing tags.
    #[serde(rename = "$text")]
    Text(String),

    /// Represents an HTML `<sub>` (subscript) element.
    Sub {
        /// The child elements contained within the subscript.
        #[serde(rename = "$value", default)]
        children: Vec<ContentNode>,
    },

    /// Represents an HTML `<sup>` (superscript) element.
    #[serde(alias = "inf")]
    Sup {
        /// The child elements contained within the superscript.
        #[serde(rename = "$value", default)]
        children: Vec<ContentNode>,
    },

    /// Represents an HTML `<i>` (italic) element.
    I {
        /// The child elements contained within the italic formatting.
        #[serde(rename = "$value", default)]
        children: Vec<ContentNode>,
    },

    /// Represents an HTML `<b>` (bold) element.
    #[serde(alias = "bold")]
    B {
        /// The child elements contained within the bold formatting.
        #[serde(rename = "$value", default)]
        children: Vec<ContentNode>,
    },

    /// Represents an HTML `<u>` (underline) element.
    U {
        /// The child elements contained within the underline formatting.
        #[serde(rename = "$value", default)]
        children: Vec<ContentNode>,
    },

    /// Represents an HTML `<xref>` (cross-reference) element.
    Xref {
        /// The cross-reference type (e.g., "bibr", "fig", "table").
        #[serde(rename = "@ref-type", default)]
        _ref_type: Option<String>,

        /// The target identifier being referenced.
        #[serde(rename = "@rid", default)]
        _rid: Option<String>,

        /// The child elements or text contained within the reference link.
        #[serde(rename = "$value", default)]
        children: Vec<ContentNode>,
    },

    /// Represents an HTML `<sc>` (small caps) element.
    Sc {
        /// The child elements contained within the small caps formatting.
        #[serde(rename = "$value", default)]
        children: Vec<ContentNode>,
    },

    /// Represents a MathML `<mi>` (identifier) element, typically used for
    /// variables.
    Mi {
        /// The raw textual content of the identifier.
        #[serde(rename = "$text", default)]
        text: String,
    },

    /// Represents a MathML `<mn>` (number) element.
    Mn {
        /// The raw textual content of the number.
        #[serde(rename = "$text", default)]
        text: String,
    },

    /// Represents a MathML `<mo>` (operator) element.
    Mo {
        /// The raw textual content of the operator.
        #[serde(rename = "$text", default)]
        text: String,
    },

    /// Represents a MathML `<mtext>` (text) element.
    Mtext {
        /// The raw textual content within the text node.
        #[serde(rename = "$text", default)]
        text: String,
    },

    /// Represents a MathML `<ms>` (string literal) element.
    Ms {
        /// The raw textual content of the string literal.
        #[serde(rename = "$text", default)]
        text: String,
    },

    /// Represents the root MathML `<math>` container element.
    Math {
        /// The mathematical elements contained within the formula.
        #[serde(rename = "$value", default)]
        children: Vec<ContentNode>,
    },

    /// Represents a MathML `<mrow>` element used to group sub-expressions.
    Mrow {
        /// The grouped child elements.
        #[serde(rename = "$value", default)]
        children: Vec<ContentNode>,
    },

    /// Represents a MathML `<mstyle>` element used to apply styles to its
    /// children.
    Mstyle {
        /// The child elements to which the style is applied.
        #[serde(rename = "$value", default)]
        children: Vec<ContentNode>,
    },

    /// Represents a MathML `<mpadded>` element used to adjust spatial
    /// properties.
    Mpadded {
        /// The child elements enclosed by the padded spacing.
        #[serde(rename = "$value", default)]
        children: Vec<ContentNode>,
    },

    /// Represents a MathML `<mphantom>` element used to render invisible
    /// space taking up the size of its children.
    Mphantom {
        /// The child elements dictating the phantom sizing.
        #[serde(rename = "$value", default)]
        children: Vec<ContentNode>,
    },

    /// Represents a MathML `<menclose>` element used to render an enclosure
    /// (e.g., box or strike-through) around its contents.
    Menclose {
        /// The enclosed child elements.
        #[serde(rename = "$value", default)]
        children: Vec<ContentNode>,
    },

    /// Represents a MathML `<mfenced>` element used to wrap its children in
    /// parentheses or brackets.
    Mfenced {
        /// The child elements to be fenced.
        #[serde(rename = "$value", default)]
        children: Vec<ContentNode>,
    },

    /// Represents a MathML `<mmultiscripts>` element used for complex tensors
    /// with pre/post sub/superscripts.
    Mmultiscripts {
        /// The base element followed by subscript/superscript pairs.
        #[serde(rename = "$value", default)]
        children: Vec<ContentNode>,
    },

    /// Represents a MathML `<mspace/>` element used to explicitly insert
    /// blank space.
    Mspace,

    /// Represents a MathML `<mprescripts/>` empty element, used within
    /// `<mmultiscripts>` to separate post-scripts from pre-scripts.
    Mprescripts,

    /// Represents a MathML `<none/>` empty element, used as a placeholder in
    /// scripts.
    None,

    /// Represents a MathML `<semantics>` element, associating presentation
    /// markup with semantic meanings.
    Semantics {
        /// The presentation layout and its associated semantic annotations.
        #[serde(rename = "$value", default)]
        children: Vec<ContentNode>,
    },

    /// Represents a MathML `<annotation>` element, providing alternate format
    /// encoding inside a `<semantics>` block.
    Annotation {
        /// The encoding attribute (e.g., "application/x-tex").
        #[serde(rename = "@encoding", default)]
        _encoding: Option<String>,

        /// The text content of the annotation.
        #[serde(rename = "$text", default)]
        text: String,
    },

    /// Represents a MathML `<msup>` element for attaching a superscript to a
    /// base (`base^{exp}`).
    Msup {
        /// Expects two children: the base and the exponent.
        #[serde(rename = "$value", default)]
        children: Vec<ContentNode>,
    },

    /// Represents a MathML `<msub>` element for attaching a subscript to a
    /// base (`base_{sub}`).
    Msub {
        /// Expects two children: the base and the subscript.
        #[serde(rename = "$value", default)]
        children: Vec<ContentNode>,
    },

    /// Represents a MathML `<msubsup>` element for attaching both a subscript
    /// and superscript (`base_{sub}^{sup}`).
    Msubsup {
        /// Expects three children: the base, the subscript, and the
        /// superscript.
        #[serde(rename = "$value", default)]
        children: Vec<ContentNode>,
    },

    /// Represents a MathML `<munder>` element for attaching an underscript
    /// (`\underset{under}{base}`).
    Munder {
        /// Expects two children: the base and the underscript.
        #[serde(rename = "$value", default)]
        children: Vec<ContentNode>,
    },

    /// Represents a MathML `<mover>` element for attaching an overscript
    /// (`\overset{over}{base}`).
    Mover {
        /// Expects two children: the base and the overscript.
        #[serde(rename = "$value", default)]
        children: Vec<ContentNode>,
    },

    /// Represents a MathML `<munderover>` element for attaching both an
    /// under- and overscript.
    Munderover {
        /// Expects three children: the base, the underscript, and the
        /// overscript.
        #[serde(rename = "$value", default)]
        children: Vec<ContentNode>,
    },

    /// Represents a MathML `<mfrac>` element for defining a fraction
    /// (`\frac{num}{den}`).
    Mfrac {
        /// Expects two children: the numerator and the denominator.
        #[serde(rename = "$value", default)]
        children: Vec<ContentNode>,
    },

    /// Represents a MathML `<msqrt>` element for defining a square root
    /// (`\sqrt{content}`).
    Msqrt {
        /// The contents inside the square root.
        #[serde(rename = "$value", default)]
        children: Vec<ContentNode>,
    },

    /// Represents a MathML `<mroot>` element for defining a root with an
    /// index (`\sqrt[index]{base}`).
    Mroot {
        /// Expects two children: the base and the root index.
        #[serde(rename = "$value", default)]
        children: Vec<ContentNode>,
    },

    /// Represents a MathML `<mtable>` element, used to lay out matrices or
    /// alignment arrays.
    Mtable {
        /// The rows (`<mtr>`) of the table.
        #[serde(rename = "$value", default)]
        children: Vec<ContentNode>,
    },

    /// Represents a MathML `<mtr>` element, denoting a single table row.
    Mtr {
        /// The cells (`<mtd>`) contained within the row.
        #[serde(rename = "$value", default)]
        children: Vec<ContentNode>,
    },

    /// Represents a MathML `<mtd>` element, denoting a table data cell.
    Mtd {
        /// The content contained within the cell.
        #[serde(rename = "$value", default)]
        children: Vec<ContentNode>,
    },

    /// Represents a structural element wrapping a display/block equation.
    #[serde(alias = "disp-formula", alias = "DispFormula")]
    DispFormula {
        /// The math elements forming the display formula.
        #[serde(rename = "$value", default)]
        children: Vec<ContentNode>,
    },

    /// Represents a structural element wrapping an inline equation.
    #[serde(alias = "inline-formula", alias = "InlineFormula")]
    InlineFormula {
        /// The math elements forming the inline formula.
        #[serde(rename = "$value", default)]
        children: Vec<ContentNode>,
    },
}

impl ContentNode {
    /// Converts the node tree recursively into a Markdown and LaTeX string
    /// block.
    ///
    /// This method ensures that inline HTML formatting elements map to
    /// CommonMark or extended Markdown syntax, and MathML elements accurately
    /// compile down to their closest LaTeX-flavored representation.
    ///
    /// # Returns
    /// A [`String`] containing the Markdown/LaTeX equivalent of the tree.
    fn to_markdown(&self) -> String {
        match self {
            Self::Text(s) => s.clone(),

            Self::Sub { children } => {
                let inner = join(children);
                format!("~{}~", inner)
            }
            Self::Sup { children } => {
                let inner = join(children);
                format!("^{}^", inner)
            }
            Self::I { children } => {
                let inner = join(children);
                format!("*{}*", inner)
            }
            Self::B { children } => {
                let inner = join(children);
                format!("**{}**", inner)
            }
            Self::U { children } => {
                let inner = join(children);
                format!("<u>{}</u>", inner)
            }
            Self::Sc { children } => {
                let inner = join(children);
                format!("\\textsc{{{}}}", inner)
            }

            Self::Mi { text }
            | Self::Mn { text }
            | Self::Mo { text }
            | Self::Mtext { text }
            | Self::Ms { text } => text.trim().to_string(),

            Self::Math { children }
            | Self::Mrow { children }
            | Self::Mstyle { children }
            | Self::Mpadded { children }
            | Self::Mphantom { children }
            | Self::Menclose { children }
            | Self::Mtd { children }
            | Self::Xref { children, .. } => join(children),

            Self::Msup { children } => {
                format!("{}^{{{}}}", nth(children, 0), nth(children, 1))
            }
            Self::Msub { children } => {
                format!("{}_{{{}}}", nth(children, 0), nth(children, 1))
            }
            Self::Msubsup { children } => {
                format!(
                    "{}_{{{}}}^{{{}}}",
                    nth(children, 0),
                    nth(children, 1),
                    nth(children, 2)
                )
            }
            Self::Munder { children } => {
                format!(
                    "\\underset{{{}}}{{{}}}",
                    nth(children, 1),
                    nth(children, 0)
                )
            }
            Self::Mover { children } => {
                format!(
                    "\\overset{{{}}}{{{}}}",
                    nth(children, 1),
                    nth(children, 0)
                )
            }
            Self::Munderover { children } => {
                format!(
                    "\\underset{{{}}}{{\\overset{{{}}}{{{}}}}}",
                    nth(children, 1),
                    nth(children, 2),
                    nth(children, 0)
                )
            }

            Self::Mfrac { children } => {
                format!(
                    "\\frac{{{}}}{{{}}}",
                    nth(children, 0),
                    nth(children, 1)
                )
            }
            Self::Msqrt { children } => {
                format!("\\sqrt{{{}}}", join(children))
            }
            Self::Mroot { children } => {
                format!("\\sqrt[{}]{{{}}}", nth(children, 1), nth(children, 0))
            }

            Self::Mfenced { children } => {
                let inner = children
                    .iter()
                    .map(|c| c.to_markdown())
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("({})", inner)
            }

            Self::Mmultiscripts { children } => {
                if children.is_empty() {
                    return String::new();
                }

                let base = children[0].to_markdown();

                let mut parts =
                    children[1..].split(|c| matches!(c, Self::Mprescripts));

                let post_scripts = parts.next().unwrap_or(&[]);
                let pre_scripts = parts.next().unwrap_or(&[]);

                let format_pair = |pair: &[Self]| {
                    let mut out = String::new();
                    if let Some(sub) = pair.first()
                        && !matches!(sub, Self::None)
                    {
                        out.push_str(&format!("_{{{}}}", sub.to_markdown()));
                    }
                    if let Some(sup) = pair.get(1)
                        && !matches!(sup, Self::None)
                    {
                        out.push_str(&format!("^{{{}}}", sup.to_markdown()));
                    }
                    out
                };

                let pre_str = format_pair(pre_scripts);
                let post_str = format_pair(post_scripts);

                format!("{{{}}}{}{}", pre_str, base, post_str)
            }

            Self::Mprescripts | Self::None => unreachable!(),

            Self::Mspace => "\\,".to_string(),

            Self::Semantics { children } => {
                if let Some(anno) = children
                    .iter()
                    .find(|c| matches!(c, Self::Annotation { .. }))
                {
                    anno.to_markdown()
                } else {
                    join(children)
                }
            }
            Self::Annotation { text, .. } => {
                text.trim().trim_matches('$').trim().to_string()
            }

            Self::Mtable { children } => children
                .iter()
                .map(|c| c.to_markdown())
                .collect::<Vec<_>>()
                .join(" \\\\ "),

            Self::Mtr { children } => children
                .iter()
                .map(|c| c.to_markdown())
                .collect::<Vec<_>>()
                .join(" & "),

            Self::DispFormula { children } => {
                format!("$$\n{}\n$$", join(children))
            }
            Self::InlineFormula { children } => {
                format!("$\n{}\n$", join(children))
            }
        }
    }
}

/// Helper function to concatenate a slice of elements.
///
/// Iterates over the provided node slice, converting each child to its
/// Markdown representation, and returns the joined string.
///
/// # Arguments
/// * `children` - A slice containing the nodes to be concatenated.
///
/// # Returns
/// A single [`String`] containing the contiguous text representations.
fn join(children: &[ContentNode]) -> String {
    children.iter().map(|c| c.to_markdown()).collect()
}

/// Helper function to safely extract and convert the `n`-th child to
/// Markdown.
///
/// If the requested index is out of bounds, this function returns an empty
/// string instead of panicking.
///
/// # Arguments
/// * `children` - A slice containing the nodes to search.
/// * `n` - The zero-based index of the target node.
///
/// # Returns
/// A [`String`] of the formatted child element, or `""` if the index does not
/// exist.
fn nth(children: &[ContentNode], n: usize) -> String {
    children.get(n).map(|c| c.to_markdown()).unwrap_or_default()
}

/// Deserializes a data stream into a Markdown string.
///
/// # Arguments
/// * `deserializer` - The Serde deserializer instance driving the parsing
///   flow.
///
/// # Returns
/// A [`Result`] containing the fully joined Markdown text if successful.
///
/// # Errors
/// Returns a `D::Error` if the underlying deserializer fails to parse the
/// incoming data stream into a valid sequence of elements.
/// This typically happens if the XML/HTML structure is malformed or contains
/// unexpected data types.
pub fn deserialize_to_markdown<'de, D>(
    deserializer: D,
) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let root: Vec<ContentNode> = Vec::<ContentNode>::deserialize(deserializer)?;
    Ok(root.iter().map(|n| n.to_markdown()).collect())
}
