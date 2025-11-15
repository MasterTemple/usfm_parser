/**
# `\sup ...\sup*`

[Source](https://ubsicap.github.io/usfm/characters/index.html#sup-sup)

- **Syntax**: `\sup_text...\sup*`
- **Type**: `character`
- **Added**: `3.0`
- **Use**: Superscript text. Typically for use in critical edition footnotes.
*/
#[derive(crate::Marker!, crate::Deref!)]
pub struct SUP(bool);
crate::impl_paired_tag!(SUP, "sup");
