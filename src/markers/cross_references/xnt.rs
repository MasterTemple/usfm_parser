/**
# `\xnt ...\xnt\*`

[Source](https://ubsicap.github.io/usfm/notes_basic/xrefs.html#xnt-xnt)

- **Syntax**: `\xnt_refs...\xnt*`
- **Type**: `character (note)`
- **Added**: `2.2`
- **Use**: References (or other text) between these markers is material to be included only in published editions that contain the New Testament books. *(optional)*
*/
#[derive(crate::Marker!)]
pub struct XNT;
