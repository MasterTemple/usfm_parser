/**
# `\tr`

[Source](https://ubsicap.github.io/usfm/tables/index.html#tr)

- **Syntax**: `\tr_`
- **Type**: `paragraph`
- **Added**: `1.0`
- **Use**: Table row start. \
    The first `\tr` initiates a new table. \
    Rows contain column `headings` or `cells`.

*/
#[derive(crate::Marker!)]
pub struct TR;
