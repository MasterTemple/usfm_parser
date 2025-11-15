//! `[BookChapterLabel]` — [Book Chapter Label](https://docs.usfm.bible/usfm/3.1.1/doc/index.html#doc-chapter-label)

use crate::markers::markers::all::{ID, USFM};

pub struct BookChapterLabel {
    // TODO: this should be something like Content<ID>
    id: ID,
    usfm: USFM,
}
