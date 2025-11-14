/**
# `\no ...\no*`

[Source](https://ubsicap.github.io/usfm/characters/index.html#no-no)

- **Syntax**: `\no_text...\no*`
- **Type**: `character`
- **Added**: `1.0`
- **Use**: Normal text. \
    May be used when a larger paragraph element is set in an alternate font style (e.g. italic), and a selected section of text should be displayed in normal text.
*/
#[derive(crate::Marker!)]
pub struct NO;
crate::impl_paired_tag!(NO, "no");
