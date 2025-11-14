use chumsky::prelude::*;
use chumsky::text::{Char, inline_whitespace, newline, whitespace};
use chumsky::{Parser, error::Rich, extra::Err, prelude::just, text};

use crate::markers::markers::AnyMarker;

pub struct Node<'a> {
    tag: AnyMarker,
    content: Option<&'a str>,
    child: Box<Node<'a>>,
}

const BACKSLASH: char = '\\';

// /// `\? .*`
// // fn leading<'a>() -> impl Parser<'a, &'a str, (&'a str, &'a str), Err<Rich<'a, char>>> {
// fn leading<'a>() -> impl Parser<'a, &'a str, (AnyMarker, &'a str), Err<Rich<'a, char>>> {
//     let non_whitespace = any().and_is(inline_whitespace().at_least(1).not());
//     let non_whitespace_slice = non_whitespace.repeated().to_slice();
//     let non_newline = any().and_is(newline().not());
//     let non_newline_slice = non_newline.repeated().to_slice();
//
//     just(BACKSLASH)
//         // .ignore_then(non_whitespace_slice)
//         .ignore_then(non_whitespace_slice.from_str().unwrapped())
//         .then_ignore(inline_whitespace())
//         .then(non_newline_slice)
//         .then_ignore(newline().or(end()))
// }
//
// #[test]
// fn test_leading() {
//     dbg!(leading().parse("\\h Romans"));
// }
