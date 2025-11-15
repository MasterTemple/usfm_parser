#[macro_use]
extern crate macro_rules_attribute;
pub mod derive_aliases;
pub(crate) use derive_aliases::*;

pub mod book_identifiers;
pub mod categories;
pub mod document;
pub mod markers;
pub mod parser;

fn main() {}
