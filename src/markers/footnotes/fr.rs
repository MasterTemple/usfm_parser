/**
# `\fr`

[Source](https://ubsicap.github.io/usfm/notes_basic/fnotes.html#fr)

- **Syntax**: `\fr_##SEP##`
- **Type**: `character (note)`
- **Added**: `1.0`
- **Use**: Footnote origin reference. \
    This is the chapter and verse(s) that note refers to. \
    `SEP` indicates where the appropriate chapter/verse separator should be used (i.e. colon ":", full stop "." etc.)
*/
#[derive(crate::Marker!)]
pub struct FR;
crate::impl_simple_tag!(FR, "fr");
