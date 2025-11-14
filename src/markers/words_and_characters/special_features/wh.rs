/**
# `\wh ...\wh*`

[Source](https://ubsicap.github.io/usfm/characters/index.html#wh-wh)

- **Syntax**: `\wh_text...\wh*`
- **Type**: `character`
- **Added**: `1.0`
- **Use**: Hebrew word list entry.
*/
#[derive(crate::Marker!)]
pub struct WH;
crate::impl_paired_tag!(WH, "wh");
