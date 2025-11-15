/**
# `\em ...\em*`

[Source](https://ubsicap.github.io/usfm/characters/index.html#em-em)

- **Syntax**: `\em_text...\em*`
- **Type**: `character`
- **Added**: `2.0`
- **Use**: Emphasis text.
*/
#[derive(crate::Marker!, crate::Deref!)]
pub struct EM(bool);
crate::impl_paired_tag!(EM, "em");
