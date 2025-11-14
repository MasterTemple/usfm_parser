/**
# `\em ...\em*`

[Source](https://ubsicap.github.io/usfm/characters/index.html#em-em)

- **Syntax**: `\em_text...\em*`
- **Type**: `character`
- **Added**: `2.0`
- **Use**: Emphasis text.
*/
#[derive(crate::Marker!)]
pub struct EM;
crate::impl_paired_tag!(EM, "em");
