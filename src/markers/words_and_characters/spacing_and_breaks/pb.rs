/**
# `\pb`

[Source](https://ubsicap.github.io/usfm/characters/index.html#pb)

- **Syntax**: `\pb`
- **Type**: `paragraph`
- **Added**: `1.0`
- **Use**: Explicit page break.
*/
#[derive(crate::Marker!)]
pub struct PB;
crate::impl_simple_tag!(PB, "pb");
