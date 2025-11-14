/**
# `\qm#`

[Source](https://ubsicap.github.io/usfm/poetry/index.html#qm)

- **Syntax**: `\qm#(_text...)`
- **Type**: `paragraph`
- **Added**: `2.0`
- **Use**: Embedded text poetic line. \
    The variable # represents the level of indent (i.e. \qm1, \qm2 etc.). \
    **\qm = \qm1** (see `syntax notes` on numbered markers)

**Text and Formatting Sample** - 1 Chronicles 12.18 (GNT - *markup adapted*)

```usfm
\p
\v 18 God's spirit took control of one of them, Amasai, who later became the commander
of “The Thirty,” and he called out,
\qm1 “David son of Jesse, we are yours!
\qm1 Success to you and those who help you!
\qm1 God is on your side.”
\b
\m David welcomed them and made them officers in his army.
```
*/
#[derive(crate::Marker!, crate::Weighted!)]
pub struct QM(Option<u8>);
crate::impl_weighted_tag!(QM, "qm");
