/**
# `\k ...\k*`

[Source](https://ubsicap.github.io/usfm/characters/index.html#k-k)

- **Syntax**: `\k_text...\k*`
- **Type**: `character`
- **Added**: `1.0`
- **Use**: Keyword / keyterm
*/
#[derive(crate::Marker!, crate::Deref!)]
pub struct K(bool);
crate::impl_paired_tag!(K, "k");
