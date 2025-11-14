/**
# `\mr`

[Source](https://ubsicap.github.io/usfm/titles_headings/index.html#mr)

- **Syntax**: `\mr_text...`
- **Type**: `paragraph`
- **Added**: `1.0`
- **Use**: Major section reference range. \
    The text reference range listed under a major section heading.

**Text and Formatting Sample** - Psalm 1 (Book 1 division) (GNT)

```usfm
\c 1
\ms BOOK ONE
\mr (Psalms 1–41)
\s True Happiness
\q1
\v 1 Happy are those
\q2 who reject the advice of evil people,
```
*/
#[derive(crate::Marker!)]
pub struct MR;
crate::impl_simple_tag!(MR, "mr");
