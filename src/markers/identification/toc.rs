/**
# `\toc#`

[Source](https://ubsicap.github.io/usfm/identification/index.html#toc)

- **Syntax**: `\toc1_text...`
- **Type**: `paragraph`
- **Added**: `2.03, 2.04 (\toc3)`
- **Use**: Long table of contents text.

- **Syntax**: `\toc2_text...`
- **Use**: Short table of contents text.

- **Syntax**: `\toc3_text...`
- **Use**: Book abbreviation. Commonly used for books names in a list of cross-reference `target references`.

**Text and Formatting Sample** - Matthew (GNT)

```usfm
\h Matthew
\toc1 The Gospel According to Matthew
\toc2 Matthew
\toc3 Mat
```

Matthew (Spanish DHH)

```usfm
\h San Mateo
\toc1 Evangelio según San Mateo
\toc2 San Mateo
\toc3 Mt
```
*/
#[derive(crate::Marker!)]
pub struct TOC;
