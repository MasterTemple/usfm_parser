/**
# `\pmr`

[Source](https://ubsicap.github.io/usfm/paragraphs/index.html#pmr)

- **Syntax**: `\pmr_text...`
- **Type**: `paragraph`
- **Added**: `2.0`
- **Use**:     Embedded text refrain.
*/
#[derive(crate::Marker!)]
pub struct PMR;
crate::impl_simple_tag!(PMR, "pmr");
