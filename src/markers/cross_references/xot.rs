/**
# `\xot ...\\xot\*`

[Source](https://ubsicap.github.io/usfm/notes_basic/xrefs.html#xot-xot)

- **Syntax**: `\xot_refs...\xot*`
- **Type**: `character (note)`
- **Added**: `2.2`
- **Use**: References (or other text) between these markers is material to be included only in published editions that contain the Old Testament books. *(optional)*
*/
#[derive(crate::Marker!)]
pub struct XOT;
