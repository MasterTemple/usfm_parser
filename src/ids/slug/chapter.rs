use std::{fmt::Display, str::FromStr};

use chumsky::error::Rich;
use chumsky::extra::Err;
use chumsky::prelude::*;
use chumsky::text::{Char, digits, inline_whitespace, newline, whitespace};
use chumsky::{Parser, error::Simple};

use from_nested_tuple::FromTuple;
use serde::{Deserialize, Serialize};

use crate::ids::code::BookCode;

#[derive(crate::Cmp!)]
#[derive(Debug, Copy, Clone, FromTuple)]
pub struct ChapterSlug {
    book: BookCode,
    chapter: u8,
}

impl ChapterSlug {
    pub fn new(book: BookCode, chapter: u8) -> Self {
        Self { book, chapter }
    }
    pub fn book(&self) -> BookCode {
        self.book
    }
    pub fn chapter(&self) -> u8 {
        self.chapter
    }
}

impl Display for ChapterSlug {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.book, self.chapter)
    }
}

impl Serialize for ChapterSlug {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.collect_str(&self.to_string())
    }
}

impl ChapterSlug {
    pub fn parser<'a>() -> impl Parser<'a, &'a str, Self, Err<Rich<'a, char>>> {
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
            .map(FromTuple::from_tuple)
    }
}

impl FromStr for ChapterSlug {
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

impl<'de> Deserialize<'de> for ChapterSlug {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // TODO: to avoid allocation: https://github.com/serde-rs/serde/issues/908#issuecomment-2929512791
        let s = String::deserialize(deserializer)?;
        FromStr::from_str(&s).map_err(serde::de::Error::custom)
    }
}

