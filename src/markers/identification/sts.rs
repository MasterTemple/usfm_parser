/**
# `\sts`

[Source](https://ubsicap.github.io/usfm/identification/index.html#sts)

- **Syntax**: `\sts_<STATUS CODE>`
- **Type**: `paragraph`
- **Added**: `1.0`
- **Use**: Project text status tracking. \
    *The contents of the status marker can be defined by the downstream system* being used to track project status. \
    Multiple status entries can be contained in a book to indicate that various portion of the text are present with different draft levels. If an entire book is complete at a given status level, only one status entry is required.

**Text and Formatting Sample**

```usfm
\sts 2
```

The text following this marker is not normally used in any formatted presentation.
*/
#[derive(crate::Marker!)]
pub struct STS;
