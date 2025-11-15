/**
# `\wj ...\wj*`

[Source](https://ubsicap.github.io/usfm/characters/index.html#wj-wj)

- **Syntax**: `\wj_text...\wj*`
- **Type**: `character`
- **Added**: `2.0`
- **Use**: Words of Jesus.
*/
#[derive(crate::Marker!, crate::Deref!)]
pub struct WJ(bool);
crate::impl_paired_tag!(WJ, "wj");
