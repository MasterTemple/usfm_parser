use std::{fmt::Display, str::FromStr};

use chumsky::error::Rich;
use chumsky::extra::Err;
use chumsky::prelude::*;
use chumsky::text::{Char, digits, inline_whitespace, newline, whitespace};
use chumsky::{Parser, error::Simple};

use from_nested_tuple::FromTuple;
use serde::{Deserialize, Serialize};

use crate::ids::code::BookCode;
use super::{chapter::ChapterSlug, verse::VerseSlug};


#[derive(derive_more::From, Debug, Clone)]
#[derive(crate::Cmp!)]
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum SlugId {
    Chapter(ChapterSlug),
    Verse(VerseSlug),
}

impl SlugId {
    pub fn new_chapter(book: BookCode, chapter: u8) -> Self {
        Self::Chapter(ChapterSlug::new(book, chapter))
    }

    pub fn new_verse(book: BookCode, chapter: u8, verse: u8) -> Self {
        Self::Verse(VerseSlug::new(book, chapter, verse))
    }

    pub fn book(&self) -> BookCode {
        match self {
            SlugId::Chapter(s) => s.book(),
            SlugId::Verse(s) => s.book(),
        }
    }

    pub fn chapter(&self) -> u8 {
        match self {
            SlugId::Chapter(s) => s.chapter(),
            SlugId::Verse(s) => s.chapter(),
        }
    }

    pub fn verse(&self) -> Option<u8> {
        match self {
            SlugId::Chapter(_) => None,
            SlugId::Verse(s) => Some(s.verse()),
        }
    }
}

impl SlugId {
    fn parser<'a>() -> impl Parser<'a, &'a str, Self, Err<Rich<'a, char>>> {
        VerseSlug::parser()
            .map(SlugId::from)
            .or(ChapterSlug::parser().map(SlugId::from))
    }
}

impl Display for SlugId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SlugId::Chapter(s) => s.fmt(f),
            SlugId::Verse(s) => s.fmt(f),
        }
    }
}

// impl Serialize for SlugId {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: serde::Serializer,
//     {
//         serializer.collect_str(&self.to_string())
//     }
// }

impl FromStr for SlugId {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Pad here because this function will likely be used on its own
        Self::parser()
            .padded()
            .parse(s)
            .into_result()
            .map_err(|e| e.first().unwrap().to_string())
    }
}

// impl<'de> Deserialize<'de> for SlugId {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where
//         D: serde::Deserializer<'de>,
//     {
//         // TODO: to avoid allocation: https://github.com/serde-rs/serde/issues/908#issuecomment-2929512791
//         let s = String::deserialize(deserializer)?;
//         FromStr::from_str(&s).map_err(serde::de::Error::custom)
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slug_enum() {
        dbg!(SlugId::from_str("GEN 1"));
        dbg!(SlugId::from_str("GEN 1:"));
        dbg!(SlugId::from_str("GEN 1:2"));
        dbg!(SlugId::from_str("GEN 1:1"));
        dbg!(SlugId::from_str("GEN Z"));
        dbg!(SlugId::from_str("GEN "));
        dbg!(SlugId::from_str("GEN"));
        dbg!(SlugId::from_str("GEN 1:"));
        dbg!(SlugId::from_str("GEN 1: "));
        dbg!(SlugId::from_str("GEN 1:Z"));
    }

}