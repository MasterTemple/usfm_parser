/**
# `\bd ...\bd*`

[Source](https://ubsicap.github.io/usfm/characters/index.html#bd-bd)

- **Syntax**: `\bd_text...\bd*`
- **Type**: `character`
- **Added**: `1.0`
- **Use**: Bold text.
*/
#[derive(crate::Marker!, crate::Deref!)]
pub struct BD(bool);
crate::impl_paired_tag!(BD, "bd");
