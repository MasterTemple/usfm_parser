use chumsky::prelude::*;
use chumsky::text::{Char, digits, inline_whitespace, newline, whitespace};
use chumsky::{Parser, error::Rich, extra::Err, prelude::just, text};
use from_nested_tuple::FromTuple;

use crate::markers::markers::parameters::MarkerParameters;

#[derive(Clone, Debug, from_nested_tuple::FromTuple)]
pub struct MarkerComponents<'a> {
    marker: &'a str,
    digits: Option<u8>,
    has_asterisk: bool,
}

impl<'a> MarkerComponents<'a> {
    pub fn marker(&self) -> &'a str {
        self.marker
    }

    pub fn digits(&self) -> Option<u8> {
        self.digits
    }

    pub fn has_asterisk(&self) -> bool {
        self.has_asterisk
    }

    pub fn parameters(&self) -> MarkerParameters {
        MarkerParameters::new(self.digits, self.has_asterisk)
    }
}

impl<'a> MarkerComponents<'a> {
    pub fn parser() -> impl Parser<'a, &'a str, MarkerComponents<'a>> {
        just('\\')
            // .ignore_then(text::ident().to_slice())
            .ignore_then(
                any()
                    .filter(|c: &char| c.is_alphabetic())
                    .repeated()
                    .at_least(1)
                    .to_slice(),
            )
            .then(
                digits(10)
                    .to_slice()
                    .map(|n: &'a str| n.parse::<u8>().unwrap())
                    .or_not(),
            )
            .then(just('*').or_not().map(|e| e.is_some()))
            .map(FromTuple::from_tuple)
    }
}

#[test]
fn marker_components() {
    dbg!(MarkerComponents::parser().parse("\\simple"));
    dbg!(MarkerComponents::parser().parse("\\weighted3"));
    dbg!(MarkerComponents::parser().parse("\\open"));
    dbg!(MarkerComponents::parser().parse("\\close*"));
}
