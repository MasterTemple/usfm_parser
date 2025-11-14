/**
# `\ph#`

[Source](https://ubsicap.github.io/usfm/paragraphs/index.html#ph)

- **Syntax**: `\ph#(_text...)`
- **Type**: `paragraph`
- **Added**: `1.0`
- **Use**: Indented paragraph with hanging indent. \
    The variable # represents the level of overall paragraph indent. \
    **\\ph = \\ph1** (see `syntax notes` on numbered markers) \
    **Deprecated** (i.e. use is strongly discouraged). \
    *Recommended alternate:* `\\li#`
*/
#[derive(crate::Marker!)]
pub struct PH;
