use chumsky::prelude::*;
use chumsky::text::{Char, inline_whitespace, newline, whitespace};
use chumsky::{Parser, error::Rich, extra::Err, prelude::just, text};
use strum::EnumString;

pub mod book_identifiers;
pub mod categories;
pub mod markers;

pub trait SimpleTag {
    fn simple_tag() -> &'static str;
}

pub trait WeightedTag {
    fn weighted_tag(&self) -> String;
}

pub trait PairedTag {
    fn paired_tag(&self) -> String;
}

#[derive(Clone, Debug, EnumString)]
pub enum Tag {
    /// https://ubsicap.github.io/usfm/identification/index.html#id
    #[strum(serialize = "id")]
    Id,
    #[strum(serialize = "usfm")]
    USFM,
    #[strum(serialize = "ide")]
    IDE,
    #[strum(serialize = "sts")]
    Status,
    #[strum(serialize = "rem")]
    Remark,
    #[strum(serialize = "h")]
    Header,
    /// TODO: how should I pair FromStr with `toc1`, `toc2`, and `toc3`?
    #[strum(serialize = "toc")]
    TableOfContents,
    #[strum(serialize = "toca")]
    TableOfContentsAlt,
}

pub struct Node<'a> {
    tag: Tag,
    content: Option<&'a str>,
    child: Box<Node<'a>>,
}

/// `\? .*`
// fn leading<'a>() -> impl Parser<'a, &'a str, (&'a str, &'a str), Err<Rich<'a, char>>> {
fn leading<'a>() -> impl Parser<'a, &'a str, (Tag, &'a str), Err<Rich<'a, char>>> {
    let non_whitespace = any().and_is(inline_whitespace().at_least(1).not());
    let non_whitespace_slice = non_whitespace.repeated().to_slice();
    let non_newline = any().and_is(newline().not());
    let non_newline_slice = non_newline.repeated().to_slice();

    just('\\')
        // .ignore_then(non_whitespace_slice)
        .ignore_then(non_whitespace_slice.from_str().unwrapped())
        .then_ignore(inline_whitespace())
        .then(non_newline_slice)
        .then_ignore(newline().or(end()))
}

fn main() {
    dbg!(leading().parse("\\h Romans"));
}
