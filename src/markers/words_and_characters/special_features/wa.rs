/**
# `\wa ...\wa*`

[Source](https://ubsicap.github.io/usfm/characters/index.html#wa-wa)

- **Syntax**: `\wa_text...\wa*`
- **Type**: `character`
- **Added**: `3.0`
- **Use**: Aramaic word list entry.
*/
#[derive(crate::Marker!)]
pub struct WA;
crate::impl_paired_tag!(WA, "wa");
