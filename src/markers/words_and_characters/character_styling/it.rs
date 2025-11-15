/**
# `\it ...\it*`

[Source](https://ubsicap.github.io/usfm/characters/index.html#it-it)

- **Syntax**: `\it_text...\it*`
- **Type**: `character`
- **Added**: `1.0`
- **Use**: Italic text.
*/
#[derive(crate::Marker!, crate::Deref!)]
pub struct IT(bool);
crate::impl_paired_tag!(IT, "it");
