/**
# `\th#`

[Source](https://ubsicap.github.io/usfm/tables/index.html#th)

- **Syntax**: `\th#_text...`
- **Type**: `character`
- **Added**: `1.0 (updated 3.0 - column spanning)`
- **Use**: Table column heading. \
    The variable # represents the table column number. \ \
    |badge_3.0| Use a dash `-` between a range of column numbers to indicate that the columns should be spanned.
*/
#[derive(crate::Marker!, crate::Weighted!)]
pub struct TH(Option<u8>);
crate::impl_weighted_tag!(TH, "th");
