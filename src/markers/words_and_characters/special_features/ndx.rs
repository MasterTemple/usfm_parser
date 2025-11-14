/**
# `\ndx ...\ndx*`

[Source](https://ubsicap.github.io/usfm/characters/index.html#ndx-ndx)

- **Syntax**: `\ndx_text...\ndx*`
- **Type**: `character`
- **Added**: `1.0`
- **Use**: Subject index entry. \
    Surround word(s) with this markup to indicate that it appears (or should appear) in the subject index.
*/
#[derive(crate::Marker!)]
pub struct NDX;
crate::impl_paired_tag!(NDX, "ndx");
