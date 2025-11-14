/**
# `\pn ...\pn*`

[Source](https://ubsicap.github.io/usfm/characters/index.html#pn-pn)

- **Syntax**: `\pn_text...\pn*`
- **Type**: `character`
- **Added**: `1.0`
- **Use**: Proper name.
*/
#[derive(crate::Marker!)]
pub struct PN;
crate::impl_paired_tag!(PN, "pn");
