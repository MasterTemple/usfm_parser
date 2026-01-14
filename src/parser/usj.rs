use std::{fmt::Display, str::FromStr};

use chumsky::{Parser, error::Simple};
use from_nested_tuple::FromTuple;
use serde::{Deserialize, Serialize};

use crate::ids::{
    code::BookCode,
    slug::{chapter::ChapterSlug, either::SlugId, verse::VerseSlug},
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UsjDocument {
    /// The kind of node/element/marker this is.
    pub r#type: String,

    /// The USJ spec version.
    pub version: String,

    /// The JSON representation of scripture contents from USFM/USX.
    pub content: Vec<ContentItem>,
}

// TODO: If I make MarkerObject an enum, then it can hold the Text variant

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

// /// The definition that lives under `#/$defs/markerObject`.
// #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
// pub struct MarkerObject {
//     /// The kind/category of node or element this is,
//     /// corresponding the USFM marker and USX node.
//     pub r#type: String,
//
//     /// The corresponding marker in USFM or style in USX.
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub marker: Option<String>,
//
//     /// Child content – a heterogeneous array that may contain plain strings
//     /// or further `MarkerObject`s (recursive).
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub content: Option<Vec<ContentItem>>,
//
//     /// Indicates the Book‑chapter‑verse value in the paragraph‑based structure.
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub sid: Option<SlugId>,
//
//     /// Chapter number or verse number.
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub number: Option<String>,
//
//     /// The 3‑letter book code in `id` element.
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub code: Option<BookCode>,
//
//     /// Alternate chapter number or verse number.
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub altnumber: Option<String>,
//
//     /// Published character of chapter or verse.
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub pubnumber: Option<String>,
//
//     /// Caller character for footnotes and cross‑refs.
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub caller: Option<String>,
//
//     /// Alignment of table cells.
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub align: Option<String>,
//
//     /// Category of extended study‑bible sections.
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub category: Option<String>,
// }

/// The definition that lives under `#/$defs/markerObject`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum MarkerObject {
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
        sid: ChapterSlug,
    },
    #[serde(rename = "verse")]
    Verse {
        marker: String,
        number: String,
        /// Indicates the Book‑chapter‑verse value in the paragraph‑based structure.
        sid: VerseSlug,
    },
    #[serde(rename = "para")]
    Paragraph {
        marker: String,
        content: Vec<ContentItem>,
    },
    #[serde(rename = "char")]
    Character {
        marker: String,
        content: Vec<ContentItem>,
    },
    #[serde(rename = "ms")]
    Milestone { marker: String },
    #[serde(rename = "note")]
    Note {
        marker: String,
        content: Vec<ContentItem>,
        /// Caller character for footnotes and cross‑refs.
        caller: String,
    },
    /*
    TODO: What about these fields?

    /// Alternate chapter number or verse number.
    altnumber: Option<String>,
    /// Published character of chapter or verse.
    pubnumber: Option<String>,
    /// Caller character for footnotes and cross‑refs.
    caller: Option<String>,
    /// Alignment of table cells.
    align: Option<String>,
    /// Category of extended study‑bible sections.
    category: Option<String>,
    */
}

impl MarkerObject {
    // TODO: Why is this optional? It seems always present
    /// The corresponding marker in USFM or style in USX.
    pub fn marker(&self) -> Option<&String> {
        Some(match self {
            MarkerObject::Book { marker, .. } => marker,
            MarkerObject::Chapter { marker, .. } => marker,
            MarkerObject::Verse { marker, .. } => marker,
            MarkerObject::Paragraph { marker, .. } => marker,
            MarkerObject::Character { marker, .. } => marker,
            MarkerObject::Milestone { marker } => marker,
            MarkerObject::Note { marker, .. } => marker,
        })
    }

    /// Child content – a heterogeneous array that may contain plain strings
    /// or further `MarkerObject`s (recursive).
    pub fn content(&self) -> Option<&Vec<ContentItem>> {
        Some(match self {
            MarkerObject::Book { content, .. } => content,
            MarkerObject::Paragraph { content, .. } => content,
            MarkerObject::Character { content, .. } => content,
            MarkerObject::Note { content, .. } => content,
            _ => None?,
        })
    }

    /// Indicates the Book‑chapter‑verse value in the paragraph‑based structure.
    pub fn sid(&self) -> Option<SlugId> {
        Some(match self {
            MarkerObject::Chapter { sid, .. } => SlugId::from(*sid),
            MarkerObject::Verse { sid, .. } => SlugId::from(*sid),
            _ => None?,
        })
    }

    /// Chapter number or verse number.
    pub fn number(&self) -> Option<&String> {
        Some(match self {
            MarkerObject::Chapter { number, .. } => number,
            MarkerObject::Verse { number, .. } => number,
            _ => None?,
        })
    }

    /// The 3‑letter book code in `id` element.
    pub fn code(&self) -> Option<BookCode> {
        Some(match self {
            MarkerObject::Book { code, .. } => *code,
            _ => None?,
        })
    }

    /// Alternate chapter number or verse number.
    pub fn altnumber(&self) -> Option<String> {
        todo!()
    }

    /// Published character of chapter or verse.
    pub fn pubnumber(&self) -> Option<String> {
        todo!()
    }

    /// Caller character for footnotes and cross‑refs.
    pub fn caller(&self) -> Option<&String> {
        Some(match self {
            MarkerObject::Note { caller, .. } => caller,
            _ => None?,
        })
    }

    /// Alignment of table cells.
    pub fn align(&self) -> Option<String> {
        todo!()
    }

    /// Category of extended study‑bible sections.
    pub fn category(&self) -> Option<String> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn marker_object() {
        let res = serde_json::to_string_pretty(&MarkerObject::Book {
            marker: "id".to_string(),
            code: BookCode::Genesis,
            content: vec![],
        })
        .unwrap();
        println!("{res}");

        let res = serde_json::to_string_pretty(&MarkerObject::Chapter {
            marker: "id".to_string(),
            number: "1".into(),
            sid: ChapterSlug::new(BookCode::Genesis, 1),
        })
        .unwrap();
        println!("{res}");
        // { type: 'chapter', marker: 'c', number: '1', sid: 'GEN 1' }
    }
    // #[test]
    // fn parser2() {
    //     dbg!(SlugId::parser2().parse("GEN 1").into_result());
    //     dbg!(SlugId::parser2().parse("GEN 1:1").into_result());
    //     dbg!(SlugId::parser2().parse("GEN Z").into_result());
    //     dbg!(SlugId::parser2().parse("GEN ").into_result());
    //     dbg!(SlugId::parser2().parse("GEN").into_result());
    //     dbg!(SlugId::parser2().parse("GEN 1:").into_result());
    //     dbg!(SlugId::parser2().parse("GEN 1: ").into_result());
    //     dbg!(SlugId::parser2().parse("GEN 1:Z").into_result());
    // }
}
