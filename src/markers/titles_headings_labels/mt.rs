/**
# `\mt#`

[Source](https://ubsicap.github.io/usfm/titles_headings/index.html#mt)

- **Syntax**: `\mt#_text...`
- **Type**: `paragraph`
- **Added**: `1.0`
- **Use**: Major title. \
    The key components in the title of a biblical book. \
    The variable # represents a portion of the title, with the lesser emphasis (relative weighting) being on the higher numbers. \
    **\mt = \mt1** (see `syntax notes` on numbered markers)

**Text and Formatting Samples** - Introduction to Acts (GNT)

```usfm
\h Acts
\toc1 The Acts of the Apostles
\toc2 Acts
\mt1 THE ACTS
\mt2 of the Apostles
\is Introduction
\ip \bk The Acts of the Apostles\bk* is a continuation of \bk The Gospel according to Luke\bk*.
```

Introduction to John (GNT)

```usfm
\h John
\toc1 The Gospel according to John
\toc2 John
\mt2 The Gospel
\mt3 according to
\mt1 JOHN
\is Introduction
```
*/
#[derive(crate::Marker!)]
pub struct MT;
