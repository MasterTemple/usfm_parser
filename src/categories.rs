use strum::EnumString;

/**
# [General Syntax](https://ubsicap.github.io/usfm/about/syntax.html#id1)

- There are three broad categories of USFM markup - **paragraph**, **character**, and **note** types.
- All USFM markers begin with a backslash character `\`.
- [Paragraph](https://ubsicap.github.io/usfm/paragraphs/index.html) markers end with the next space character.
- [Character](https://ubsicap.github.io/usfm/characters/index.html) markers occur in pairs, marking a span of text within a paragraph.
- Note markers also occur in pairs, marking the start and end of the [footnote](https://ubsicap.github.io/usfm/notes_basic/fnotes.html), [cross reference](https://ubsicap.github.io/usfm/notes_basic/xrefs.html), or [study note](https://ubsicap.github.io/usfm/notes_study/index.html) content.
- For marker pairs (character and note), the opening marker ends with the next space character (as with paragraph markers). The matching closing marker is identical to the opening marker but ends with an asterisk character `*`. Example: `\w grace\w*`.
*/
#[derive(Clone, Debug, EnumString)]
pub enum Category {
    Paragraph,
    Character,
    Note,
}
