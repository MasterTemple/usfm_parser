/**
# `\mte#`

[Source](https://ubsicap.github.io/usfm/titles_headings/index.html#mte)

- **Syntax**: `\mte#_text...`
- **Type**: `paragraph`
- **Added**: `1.0`
- **Use**: Major title at ending. \
    May be used in texts which repeat the main title at the end of the introduction, or to mark a major title indicating the end of the introduction. \
    The content is not typically identical to `\mt#`. \
    The variable # represents a portion of the title, with the lesser emphasis (relative weighting) being on the higher numbers. \
    **\mte = \mte1** (see `syntax notes` on numbered markers)

**Text and Formatting Sample** - John

```usfm
\mte2 The End of the Gospel according to
\mte1 John
```
*/
#[derive(crate::Marker!)]
pub struct MTE;
