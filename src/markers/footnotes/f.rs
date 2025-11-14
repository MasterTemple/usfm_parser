/**
# `\f ...\\f\*`

[Source](https://ubsicap.github.io/usfm/notes_basic/fnotes.html#f-f)

- **Syntax**: `\f_+_(\fr_REF_)footnote content\f*`
- **Type**: `note`
- **Added**: `1.0`
- **Use**: Beginning and ending of the footnote element.

- The footnote caller, which may be one of the following three types:
    - `+` – indicates that the caller should be generated automatically by the translation editor, or publishing tools. \
    - `-` – indicates that no caller should be generated, and is not used. \
    - `?` – where ? represents the character to be used for the caller. The caller is defined for the specific note by the author.

- `footnote content` (see `below`)
    - All of the text elements which make up the footnote:
        - `origin` reference
        - special footnote elements such as keywords, quotations, alternate renderings etc.
        - footnote `text`
    - Each element should be prefixed by the appropriate marker (listed below).

## Note

> **Important:** See `Syntax Notes` for addition information on the use of `endmarkers` for elements within footnote content.

## Endnote Syntax

> Notes which are intended as "Endnotes" should be marked using the following alternative format:
*/
#[derive(crate::Marker!)]
pub struct F;
