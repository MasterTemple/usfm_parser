/**
# `\fl`

[Source](https://ubsicap.github.io/usfm/notes_basic/fnotes.html#fl)

- **Syntax**: `\fl_text...`
- **Type**: `character (note)`
- **Added**: `2.03`
- **Use**: Footnote label text. \
    Can be used for marking or "labeling" a word or words which are used consistently across certain types of translation notes (such as the words "Or" in an alternative translation note, "Others", "Heb.", "LXX" etc.).
*/
#[derive(crate::Marker!)]
pub struct FL;
