/**
# `\imte#`

[Source](https://ubsicap.github.io/usfm/introductions/index.html#imte)

- **Syntax**: `\imte#_text...`
- **Type**: `paragraph`
- **Added**: `1.0`
- **Use**: Introduction major title ending. \
    Used to mark a major title indicating the end of the introduction. \
    The variable # represents a portion of the title, with the lesser emphasis (relative weighting) being on the higher numbers. \
    **\imte = \imte1** (see `syntax notes` on numbered markers)

**Text and Formatting Sample** - Introduction to Mark

```usfm
\imte End of the Introduction to the Gospel of Mark
```
*/
#[derive(crate::Marker!, crate::Deref!)]
pub struct IMTE(Option<u8>);
crate::impl_weighted_tag!(IMTE, "imte");
