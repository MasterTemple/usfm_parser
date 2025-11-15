/**
# `\lik ...\lik*`

- **Syntax**: `\lik text...\lik*`
- **Type**: `character`
- **Added**: `3.0`
- **Use**: List entry "key" content.
*/
#[derive(crate::Marker!, crate::Deref!)]
pub struct LIK(bool);
crate::impl_paired_tag!(LIK, "lik");
