/**
# `\ib`

[Source](https://ubsicap.github.io/usfm/introductions/index.html#ib)

- **Syntax**: `\ib`
- **Type**: `paragraph`
- **Added**: `1.0`
- **Use**: Introduction blank line. \
    May be used to explicitly indicate additional white space between paragraphs.

**Text and Formatting Sample** - Introduction to Genesis (CEV)

```usfm
... One of these brothers, Joseph, had become the governor of Egypt. But Joseph knew that
God would someday keep his promise to his people:
\ib
\imq Before Joseph died, he told his brothers, “I won't live much longer. But God will take
care of you and lead you out of Egypt to the land he promised Abraham, Isaac, and Jacob.”
```

*See also* `\ipq`, `\imq` examples (above).
*/
#[derive(crate::Marker!)]
pub struct IB;
crate::impl_simple_tag!(IB, "ib");
