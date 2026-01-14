use std::{fmt::Display, str::FromStr};

use chumsky::{Parser, error::Simple};
use from_nested_tuple::FromTuple;
use serde::{Deserialize, Serialize};

use crate::ids::code::BookCode;

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

impl SlugId {
    pub fn chapter(book: BookCode, chapter: u8) -> Self {
        Self {
            book,
            chapter,
            verse: None,
        }
    }

    pub fn verse(book: BookCode, chapter: u8, verse: u8) -> Self {
        Self {
            book,
            chapter,
            verse: Some(verse),
        }
    }
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

#[cfg(test)]
mod tests {
    use super::*;
    // #[test]
    // fn marker_object() {
    //     let res = serde_json::to_string_pretty(&MarkerObjectEnum::Book {
    //         marker: "id".to_string(),
    //         code: BookCode::Genesis,
    //         content: vec![],
    //     })
    //     .unwrap();
    //     println!("{res}");
    //
    //     let res = serde_json::to_string_pretty(&MarkerObjectEnum::Chapter {
    //         marker: "id".to_string(),
    //         number: "1".into(),
    //         sid: SlugId {
    //             book: BookCode::Genesis,
    //             chapter: 1,
    //             verse: None,
    //         },
    //     })
    //     .unwrap();
    //     println!("{res}");
    //     // { type: 'chapter', marker: 'c', number: '1', sid: 'GEN 1' }
    // }
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
