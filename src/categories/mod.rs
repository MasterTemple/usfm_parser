use enum_dispatch::enum_dispatch;
use strum::EnumString;

/**
# [General Syntax](https://ubsicap.github.io/usfm/about/syntax.html#id1)

- There are three broad categories of USFM markup - **paragraph**, **character**, and **note** types.
- All USFM markers begin with a backslash character `\`.
- [Paragraph](https://ubsicap.github.io/usfm/paragraphs/index.html) markers end with the next space character.
- [Character](https://ubsicap.github.io/usfm/characters/index.html) markers occur in pairs, marking a span of text within a paragraph.
- Note markers also occur in pairs, marking the start and end of the [footnote](https://ubsicap.github.io/usfm/notes_basic/fnotes.html), [cross reference](https://ubsicap.github.io/usfm/notes_basic/xrefs.html), or [study note](https://ubsicap.github.io/usfm/notes_study/index.html) content.
- For marker pairs (character and note), the opening marker ends with the next space character (as with paragraph markers). The matching closing marker is identical to the opening marker but ends with an asterisk character `*`. Example: `\w grace\w*`.

---

https://docs.usfm.bible/usfm/latest/syntax.html
https://usfmgrammar.vachanengine.org/
*/
#[derive(Clone, Debug, EnumString)]
pub enum Category {
    Paragraph,
    Character,
    Note,
}

// usfm-grammar/node-usfm-parser/src/utils/markers.js
pub enum Element {
    // "book"
    Book {
        marker: String,
        code: String,
        content: Vec<Element>,
    },
    // "chapter"
    Chapter {
        marker: String,
        number: String,
        sid: String,
    },
    // "verse"
    Verse {
        marker: String,
        number: String,
        sid: String,
    },
    // "para"
    Paragraph {
        marker: String,
        content: Vec<Element>,
    },
    // "char"
    Character {
        marker: String,
        content: Vec<Element>,
    },
    // "ms"
    Milestone {
        marker: String,
    },
    // "note"
    Note {
        marker: String,
        content: Vec<Element>,
        caller: String,
    },
    Text(String),
}

// usfm-grammar/node-usfm-parser/src/filters.js
use crate::markers::markers::all::*;
#[enum_dispatch]
pub enum BookMarker {
    IDE,
    USFM,
    H,
    TOC,
    TOCA,
    IMT,
    IS,
    IP,
    IPI,
    IM,
    IMI,
    IPQ,
    IMQ,
    IPR,
    IQ,
    IB,
    ILI,
    IOT,
    IO,
    IEX,
    IMTE,
    IE,
}

/// usfm-grammar/node-usfm-parser/src/filters.js
pub mod filters;
/// usfm-grammar/node-usfm-parser/src/utils/markers.js
pub mod groups;

