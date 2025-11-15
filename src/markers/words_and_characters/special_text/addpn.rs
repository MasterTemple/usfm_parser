/**
# `\addpn ...\addpn*`

[Source](https://ubsicap.github.io/usfm/characters/index.html#addpn-addpn)

- **Syntax**: `\addpn_text...\addpn*`
- **Type**: `character`
- **Added**: `2.0`
- **Deprecated**: 3.0  :Use: Support for overlapping `\pn ...\pn*` and `\add ...\add*` occurrences in Chinese texts. \
    **Deprecated** (use is discouraged). \
    *Recommended alternate:* `Nested` `\pn ...\pn*` within `\add ...\add*`.
*/
#[derive(crate::Marker!, crate::Deref!)]
pub struct ADDPN(bool);
crate::impl_paired_tag!(ADDPN, "addpn");
