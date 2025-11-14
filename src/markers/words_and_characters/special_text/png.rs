/**
# `\png ...\png\*`

[Source](https://ubsicap.github.io/usfm/characters/index.html#png-png)

- **Syntax**: `\png_text...\png*`
- **Type**: `character`
- **Added**: `3.0`
- **Use**: Geographic proper name. \
    Particularly in China, there are various groups which have the practice of distinguishing visually between proper names of people and proper names of geographic places in published texts. Published materials may use a single underline to present proper names of people, and double underline to present proper names of geographic places. Alternatively, dotted underlines have been used for geographic proper names. \ \
    Special presentation for names can assist readers to know what the text means, especially readers who may struggle with reading skills and may be overloaded by the transliterated names.
*/
#[derive(crate::Marker!)]
pub struct PNG;
