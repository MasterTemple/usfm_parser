/**
# `\w ...\w*`

[Source](https://ubsicap.github.io/usfm/characters/index.html#w-w)

- **Syntax**: `\w_text...\w*`
- **Type**: `character`
- **Added**: `1.0`
- **Updated**: 3.0 (attributes)
- **Use**: Wordlist / glossary / dictionary entry. \
    Surround word(s) with this markup to indicate that it appears (or should appear) in the word list.
*/
#[derive(crate::Marker!, crate::Deref!)]
pub struct W(bool);
crate::impl_paired_tag!(W, "w");
