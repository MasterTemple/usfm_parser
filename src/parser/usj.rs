use std::{fmt::Display, str::FromStr};

use chumsky::{Parser, error::Simple};
use from_nested_tuple::FromTuple;
use serde::{Deserialize, Serialize};

use crate::ids::{
    code::BookCode,
    slug::{chapter::ChapterSlug, either::SlugId, verse::VerseSlug},
};

/**
Mean to model the [`USJ Schema`](https://github.com/usfm-bible/tcdocs/blob/main/grammar/usj.js)
*/
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UsjDocument {
    /// The kind of node/element/marker this is.
    pub r#type: String,

    /// The USJ spec version.
    pub version: String,

    /// The JSON representation of scripture contents from USFM/USX.
    pub content: Vec<ContentItem>,
}

/**
* - I could make [`MarkerObject`] hold the `Text` variant, but then certain methods like
`type` would now return `Option<T>` instead of `T`
- A value that can appear inside the `content` arrays.
- The schema allows either a plain string **or** a nested [`MarkerObject`].
*/
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ContentItem {
    /// Plain text.
    Text(String),

    /// A nested marker object (recursive structure).
    Marker(MarkerObject),
}

// TODO: change `marker: String` to marker enums with the appropriate subset of tags

/// The definition that lives under `#/$defs/markerObject`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum MarkerObject {
    #[serde(rename = "book")]
    Book {
        /// The corresponding marker in USFM or style in USX.
        marker: String,

        /// The 3‑letter book code in `id` element.
        code: BookCode,

        // TODO: Is this always empty for books?
        //
        /// Child content – a heterogeneous array that may contain plain strings
        /// or further `MarkerObject`s (recursive).
        content: Vec<ContentItem>,
    },

    #[serde(rename = "chapter")]
    Chapter {
        /// The corresponding marker in USFM or style in USX.
        marker: String,

        /// Chapter number or verse number.
        // TODO: Why is this is a string?
        number: String,

        /// `\ca`: Alternate chapter number
        altnumber: Option<String>,

        /// `\cp`: Published character of chapter
        pubnumber: Option<String>,

        /// Indicates the Book‑chapter‑verse value in the paragraph‑based structure.
        sid: ChapterSlug,
    },

    #[serde(rename = "verse")]
    Verse {
        /// The corresponding marker in USFM or style in USX.
        marker: String,

        /// Chapter number or verse number.
        number: String,

        /// `\va`: Alternate verse number.
        altnumber: Option<String>,

        /// `\vp`: Published character of chapter or verse.
        pubnumber: Option<String>,

        /// Indicates the Book‑chapter‑verse value in the paragraph‑based structure.
        sid: VerseSlug,
    },

    #[serde(rename = "para")]
    Paragraph {
        /// The corresponding marker in USFM or style in USX.
        marker: String,

        /// Child content – a heterogeneous array that may contain plain strings
        /// or further `MarkerObject`s (recursive).
        content: Vec<ContentItem>,
    },

    #[serde(rename = "char")]
    Character {
        /// The corresponding marker in USFM or style in USX.
        marker: String,

        /// Child content – a heterogeneous array that may contain plain strings
        /// or further `MarkerObject`s (recursive).
        content: Vec<ContentItem>,
    },

    #[serde(rename = "ms")]
    Milestone {
        /// The corresponding marker in USFM or style in USX.
        marker: String,
    },

    #[serde(rename = "note")]
    Note {
        /// The corresponding marker in USFM or style in USX.
        marker: String,

        /// Child content – a heterogeneous array that may contain plain strings
        /// or further `MarkerObject`s (recursive).
        content: Vec<ContentItem>,

        /// Caller character for footnotes and cross‑refs.
        caller: String,
    },
    // TODO: I see `'table:cell'` and `'table:row'`
    #[serde(rename = "table")]
    Table {
        /// Child content – a heterogeneous array that may contain plain strings
        /// or further `MarkerObject`s (recursive).
        content: Vec<ContentItem>,
    },
    #[serde(rename = "table:row")]
    TableRow {
        /// The corresponding marker in USFM or style in USX.
        marker: String,

        /// Child content – a heterogeneous array that may contain plain strings
        /// or further `MarkerObject`s (recursive).
        content: Vec<ContentItem>,
    },
    #[serde(rename = "table:cell")]
    TableCell {
        /// The corresponding marker in USFM or style in USX.
        marker: String,

        /// Child content – a heterogeneous array that may contain plain strings
        /// or further `MarkerObject`s (recursive).
        content: Vec<ContentItem>,
        /// Alignment of table cells.
        // usjGenerator:283
        align: String,
    },
    // TODO: Not sure what goes in here, or if I am misreading `nodeToUSJSpecial` at `nodeToUSJSpecial.js:358` wrong
    #[serde(rename = "figure")]
    Figure {
        /// The corresponding marker in USFM or style in USX.
        marker: String,

        /// Child content – a heterogeneous array that may contain plain strings
        /// or further `MarkerObject`s (recursive).
        content: Vec<ContentItem>,
    },
    // TODO: Not sure what goes in here, or if I am misreading `nodeToUSJSpecial` at `nodeToUSJSpecial.js:358` wrong
    #[serde(rename = "ref")]
    Ref {
        /// The corresponding marker in USFM or style in USX.
        marker: String,

        /// Child content – a heterogeneous array that may contain plain strings
        /// or further `MarkerObject`s (recursive).
        content: Vec<ContentItem>,
    },
    // TODO: Not sure what goes in here, or if I am misreading `nodeToUSJSpecial` at `nodeToUSJSpecial.js:358` wrong
    #[serde(rename = "cat")]
    Category {},
    // TODO: Not sure what goes in here, or if I am misreading `nodeToUSJSpecial` at `nodeToUSJSpecial.js:358` wrong
    #[serde(rename = "esb")]
    ESB {},
    // TODO: Not sure what goes in here, or if I am misreading `nodeToUSJSpecial` at `nodeToUSJSpecial.js:358` wrong
    #[serde(rename = "sidebar")]
    Sidebar {},
}

impl MarkerObject {
    // TODO: Change to [`AnyMarker`] once impl'd
    //
    /// The kind/category of node or element this is, corresponding the USFM marker and USX node.
    pub fn r#type(&self) -> String {
        todo!()
    }

    // TODO: Why is this optional? It seems always present
    //
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
            MarkerObject::TableRow { marker, .. } => marker,
            MarkerObject::TableCell { marker, .. } => marker,
            MarkerObject::Figure { marker, .. } => marker,
            MarkerObject::Ref { marker, .. } => marker,
            _ => None?,
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
            altnumber: None,
            pubnumber: None,
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
