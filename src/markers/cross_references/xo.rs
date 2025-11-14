/**
# `\xo`

[Source](https://ubsicap.github.io/usfm/notes_basic/xrefs.html#xo)

- **Syntax**: `\xo_##SEP##`
- **Type**: `character (note)`
- **Added**: `1.0`
- **Use**: Cross reference origin reference. \
     This is the chapter and verse(s) that `target reference(s)` are being provided for. \
     `SEP` indicates where the appropriate chapter/verse separator should be used (i.e. colon ":", full stop "." etc.)
*/
#[derive(crate::Marker!)]
pub struct XO;
