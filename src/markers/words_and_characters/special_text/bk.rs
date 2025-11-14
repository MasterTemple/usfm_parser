/**
# `\bk ...\\bk\*`

[Source](https://ubsicap.github.io/usfm/characters/index.html#bk-bk)

- **Syntax**: `\bk_text...\bk*`
- **Type**: `character`
- **Added**: `1.0`
- **Use**: Quoted book title.

**Text and Formatting Sample** - Introduction to Mark (GNT)

```usfm
\mt1 THE ACTS
\mt2 of the Apostles
\is Introduction
\ip \bk The Acts of the Apostles\bk* is a continuation of \bk The Gospel according
to Luke\bk* Its chief purpose is to tell how Jesus' early followers, led by the Holy
Spirit, spread the Good News about him “in Jerusalem, in all of Judea and Samaria,
and to the ends of the earth” (1.8).
```
*/
#[derive(crate::Marker!)]
pub struct BK;
