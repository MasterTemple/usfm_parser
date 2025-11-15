use chumsky::prelude::*;
use chumsky::text::{Char, digits, inline_whitespace, newline, whitespace};
use chumsky::{Parser, error::Rich, extra::Err, prelude::just, text};
use from_nested_tuple::FromTuple;

// use crate::markers::markers::MarkerParsing;
use crate::markers::markers::any::AnyMarker;

pub enum NodeType {
    Leaf,
    SomeContent,
    InBetweenContent,
}

// TODO:
// - There should be a text node (which isn't denoted by a blackslash)
// - There is `~` and `//`, which have no content; so I should probably have
//
/*
Recursion:
- https://docs.rs/chumsky/latest/chumsky/recursive/fn.recursive.html
- https://github.com/zesterer/chumsky/blob/main/examples/nested.rs
- https://github.com/zesterer/chumsky/blob/main/examples/json.rs
*/
pub struct Node<'a> {
    tag: AnyMarker,
    // perhaps there should be a text node, and not a dedicated field
    content: Option<&'a str>,
    // child: Box<Node<'a>>,
    children: Vec<Node<'a>>,
}

pub struct Document<'a> {
    content: String, // could be Cow<> or Rc<>
    nodes: Vec<Node<'a>>,
}

const BACKSLASH: char = '\\';

// fn marker<'a>() -> impl Parser<'a, &'a str, MarkerParsing<'a>> {
//     just(BACKSLASH)
//         .ignore_then(text::ident().to_slice())
//         .then(
//             digits(10)
//                 .to_slice()
//                 .map(|n: &'a str| n.parse::<u8>().unwrap())
//                 .or_not(),
//         )
//         .then(just('*').or_not().map(|e| e.is_some()))
//         .map(FromTuple::from_tuple)
// }

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
//     dbg!(leading().parse("\h Romans"));
// }
