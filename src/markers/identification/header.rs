/**
# `\h`

[Source](https://ubsicap.github.io/usfm/identification/index.html#h)

- **Syntax**: `\h_text...`
- **Type**: `paragraph`
- **Added**: `1.0`
- **Amended**: 3.0
- **Use**: Running header text. \
    **Deprecated** use of numbered variable syntax (i.e. use is strongly discouraged). |badge_3.0| \
    The variable # in `\h#_text...` represented distinct components or levels of text required for the running header presentation (e.g. inside, outside, sub-division/section etc.).

**Text and Formatting Sample** - Matthew (GNT)

```usfm
\h Matthew
```
*/
pub struct Header;
