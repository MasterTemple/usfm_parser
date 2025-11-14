/**
# `\ipr`

[Source](https://ubsicap.github.io/usfm/introductions/index.html#ipr)

- **Syntax**: `\ipr_text...`
- **Type**: `paragraph`
- **Added**: `1.0`
- **Use**: Introduction right-aligned paragraph. \
    Typically used for a quote from text reference. \

**Text and Formatting Sample** - Introduction to Genesis (CEV)

```usfm
... One of these brothers, Joseph, had become the governor of Egypt. But Joseph knew that
God would someday keep his promise to his people:
\ib
\ipq Before Joseph died, he told his brothers, “I won't live much longer. But God will take
care of you and lead you out of Egypt to the land he promised Abraham, Isaac, and Jacob.”
\ipr (50.24)
\iot A QUICK LOOK AT THIS BOOK
...
```
*/
#[derive(crate::Marker!)]
pub struct IPR;
crate::impl_simple_tag!(IPR, "ipr");
