/**
# `\fp`

[Source](https://ubsicap.github.io/usfm/notes_basic/fnotes.html#fp)

- **Syntax**: `\fp_text...`
- **Type**: `character (note)`
- **Added**: `2.03`
- **Use**: Footnote additional paragraph. \
    Use this marker to if you need to indicate the start of a new paragraph within a footnote (uncommon).
*/
#[derive(crate::Marker!)]
pub struct FP;
crate::impl_simple_tag!(FP, "fp");
