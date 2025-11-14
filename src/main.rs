#[macro_use]
extern crate macro_rules_attribute;
pub mod derive_aliases;
pub(crate) use derive_aliases::*;

pub mod book_identifiers;
pub mod categories;
pub mod markers;
pub mod parser;

pub trait SimpleTag {
    fn simple_tag() -> &'static str;
}

pub trait WeightedTag {
    fn weighted_tag(&self) -> String;
}

pub trait PairedTag {
    fn paired_tag(&self) -> String;
}

fn main() {}
