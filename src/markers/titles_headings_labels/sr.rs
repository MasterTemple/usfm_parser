/**
# `\sr`

[Source](https://ubsicap.github.io/usfm/titles_headings/index.html#sr)

- **Syntax**: `\sr_text...`
- **Type**: `paragraph`
- **Added**: `2.0`
- **Use**: Section reference range. \
    The text reference range listed under a section heading. \
    \sr is not equivalent to \r which is used for marking parallel references. \
    *See also* `\mr`

**Text and Formatting Sample** - Proverbs 22.17 (GNT - *markup adapted*)

```usfm
\s1 The Thirty Wise Sayings
\sr (22.17--24.22)
\p
\v 17 Listen, and I will teach you what the wise have said. Study their teachings, ...
```
*/
#[derive(crate::Marker!)]
pub struct SR;
