use std::{fmt::Display, str::FromStr};

use chumsky::{Parser, error::Simple};
use from_nested_tuple::FromTuple;
use serde::{Deserialize, Serialize};

use crate::ids::BookCode;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UsjDocument {
    /// The kind of node/element/marker this is.
    pub r#type: String,

    /// The USJ spec version.
    pub version: String,

    /// The JSON representation of scripture contents from USFM/USX.
    pub content: Vec<ContentItem>,
}

/// A value that can appear inside the `content` arrays.
///
/// The schema allows either a plain string **or** a nested `markerObject`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ContentItem {
    /// Plain text.
    Text(String),

    /// A nested marker object (recursive structure).
    Marker(MarkerObject),
}

// #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(untagged)]
// pub enum SlugIdEnum {
//     BC(BCSlugId),
//     BCV(BCVSlugId),
// }

#[derive(Debug, Clone, PartialEq, FromTuple)]
pub struct BCSlug {
    book: BookCode,
    chapter: u8,
}

#[derive(Debug, Clone, PartialEq, FromTuple)]
pub struct BCVSlug {
    book: BookCode,
    chapter: u8,
    verse: u8,
}

// TODO: Perhaps this should be an enum
#[derive(Debug, Clone, PartialEq, FromTuple)]
pub struct SlugId {
    book: BookCode,
    chapter: u8,
    verse: Option<u8>,
}

impl Serialize for SlugId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.collect_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for SlugId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // TIP: to avoid allocation: https://github.com/serde-rs/serde/issues/908#issuecomment-2929512791
        let s = String::deserialize(deserializer)?;
        FromStr::from_str(&s).map_err(serde::de::Error::custom)
    }
}

impl SlugId {
    fn parser<'a>() -> impl Parser<'a, &'a str, SlugId> {
        use chumsky::prelude::*;
        use chumsky::text::{Char, digits, inline_whitespace, newline, whitespace};
        any()
            .filter(|c: &char| c.is_alphabetic())
            .repeated()
            .at_least(1)
            .to_slice()
            .map(|s| BookCode::from_str(s).unwrap())
            .then_ignore(inline_whitespace())
            .then(
                digits(10)
                    .to_slice()
                    .map(|n: &'a str| n.parse::<u8>().unwrap()),
            )
            .then(
                just(":")
                    .ignore_then(
                        digits(10)
                            .to_slice()
                            .map(|n: &'a str| n.parse::<u8>().unwrap()),
                    )
                    .or_not(),
            )
            .padded()
            .map(FromTuple::from_tuple)
    }

    fn parser2<'a>()
    -> impl Parser<'a, &'a str, SlugId, chumsky::extra::Err<chumsky::error::Rich<'a, char>>> {
        use chumsky::prelude::*;
        use chumsky::text::{Char, digits, inline_whitespace, newline, whitespace};
        any()
            .filter(|c: &char| c.is_alphabetic())
            .repeated()
            .at_least(1)
            .to_slice()
            .try_map(|s, span| BookCode::from_str(s).map_err(|e| Rich::custom(span, e)))
            // .then_ignore(inline_whitespace())
            .then_ignore(just(" "))
            .then(
                digits(10)
                    .to_slice()
                    // .map(|n: &'a str| n.parse::<u8>().unwrap())
                    .try_map(|n: &str, span| n.parse::<u8>().map_err(|e| Rich::custom(span, e))),
            )
            .then(
                just(":")
                    .ignore_then(
                        digits(10)
                            .to_slice()
                            // .map(|n: &'a str| n.parse::<u8>().unwrap()),
                            .try_map(|n: &str, span| {
                                n.parse::<u8>().map_err(|e| Rich::custom(span, e))
                            }),
                    )
                    .or_not(),
            )
            .padded()
            .map(FromTuple::from_tuple)
    }
}

impl Display for SlugId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(verse) = self.verse {
            write!(f, "{} {}:{}", self.book, self.chapter, verse)
        } else {
            write!(f, "{} {}", self.book, self.chapter)
        }
    }
}

impl FromStr for SlugId {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parser().parse(s).into_result().map_err(|_| "")
    }
}

/// The definition that lives under `#/$defs/markerObject`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MarkerObject {
    /// The kind/category of node or element this is,
    /// corresponding the USFM marker and USX node.
    pub r#type: String,

    /// The corresponding marker in USFM or style in USX.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,

    /// Child content – a heterogeneous array that may contain plain strings
    /// or further `MarkerObject`s (recursive).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<Vec<ContentItem>>,

    /// Indicates the Book‑chapter‑verse value in the paragraph‑based structure.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sid: Option<SlugId>,

    /// Chapter number or verse number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<String>,

    /// The 3‑letter book code in `id` element.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<BookCode>,

    /// Alternate chapter number or verse number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub altnumber: Option<String>,

    /// Published character of chapter or verse.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pubnumber: Option<String>,

    /// Caller character for footnotes and cross‑refs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caller: Option<String>,

    /// Alignment of table cells.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub align: Option<String>,

    /// Category of extended study‑bible sections.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
}

/// The definition that lives under `#/$defs/markerObject`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum MarkerObjectEnum {
    // { type: 'book', marker: 'id', code: 'GEN', content: [] },
    #[serde(rename = "book")]
    Book {
        /// The corresponding marker in USFM or style in USX.
        marker: String,

        /// The 3‑letter book code in `id` element.
        code: BookCode,

        // TODO: Why is this empty sometimes?
        //
        /// Child content – a heterogeneous array that may contain plain strings
        /// or further `MarkerObject`s (recursive).
        content: Vec<ContentItem>,
    },
    // { type: 'chapter', marker: 'c', number: '1', sid: 'GEN 1' },
    #[serde(rename = "chapter")]
    Chapter {
        /// The corresponding marker in USFM or style in USX.
        marker: String,

        /// Chapter number or verse number.
        // TODO: Why is this is a string?
        number: String,

        /// Indicates the Book‑chapter‑verse value in the paragraph‑based structure.
        // TODO: Are these always chapter-only slugs?
        sid: SlugId,
    },
    // /// Indicates the Book‑chapter‑verse value in the paragraph‑based structure.
    // #[serde(skip_serializing_if = "Option::is_none")]
    // sid: Option<String>,
    //
    //
    //
    // /// Alternate chapter number or verse number.
    // #[serde(skip_serializing_if = "Option::is_none")]
    // altnumber: Option<String>,
    //
    // /// Published character of chapter or verse.
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pubnumber: Option<String>,
    //
    // /// Caller character for footnotes and cross‑refs.
    // #[serde(skip_serializing_if = "Option::is_none")]
    // caller: Option<String>,
    //
    // /// Alignment of table cells.
    // #[serde(skip_serializing_if = "Option::is_none")]
    // align: Option<String>,
    //
    // /// Category of extended study‑bible sections.
    // #[serde(skip_serializing_if = "Option::is_none")]
    // category: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn marker_object() {
        let res = serde_json::to_string_pretty(&MarkerObjectEnum::Book {
            marker: "id".to_string(),
            code: BookCode::Genesis,
            content: vec![],
        })
        .unwrap();
        println!("{res}");

        let res = serde_json::to_string_pretty(&MarkerObjectEnum::Chapter {
            marker: "id".to_string(),
            number: "1".into(),
            sid: SlugId {
                book: BookCode::Genesis,
                chapter: 1,
                verse: None,
            },
        })
        .unwrap();
        println!("{res}");
        // { type: 'chapter', marker: 'c', number: '1', sid: 'GEN 1' }
    }
    #[test]
    fn parser2() {
        dbg!(SlugId::parser2().parse("GEN 1").into_result());
        dbg!(SlugId::parser2().parse("GEN 1:1").into_result());
        dbg!(SlugId::parser2().parse("GEN Z").into_result());
        dbg!(SlugId::parser2().parse("GEN ").into_result());
        dbg!(SlugId::parser2().parse("GEN").into_result());
        dbg!(SlugId::parser2().parse("GEN 1:").into_result());
        dbg!(SlugId::parser2().parse("GEN 1: ").into_result());
        dbg!(SlugId::parser2().parse("GEN 1:Z").into_result());
    }
}
