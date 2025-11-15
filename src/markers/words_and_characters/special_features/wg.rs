/**
# `\wg ...\wg*`

[Source](https://ubsicap.github.io/usfm/characters/index.html#wg-wg)

- **Syntax**: `\wg_text...\wg*`
- **Type**: `character`
- **Added**: `1.0`
- **Use**: Greek word list entry.
*/
#[derive(crate::Marker!, crate::Deref!)]
pub struct WG(bool);
crate::impl_paired_tag!(WG, "wg");
