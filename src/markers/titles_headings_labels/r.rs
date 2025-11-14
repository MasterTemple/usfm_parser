/**
# `\r`

[Source](https://ubsicap.github.io/usfm/titles_headings/index.html#r)

- **Syntax**: `\r_text...`
- **Type**: `paragraph`
- **Added**: `1.0`
- **Use**: Parallel passage reference(s). \
    A reference to a parallel passage usually located under a section heading `\\s#`.

**Text and Formatting Sample** - Matthew 3.1 (GNT)

```usfm
\c 3
\s1 The Preaching of John the Baptist
\r (Mark 1.1-8; Luke 3.1-18; John 1.19-28)
\p
\v 1 At that time John the Baptist came to the desert of Judea and started preaching.
\v 2 “Turn away from your sins,” he said, ...
```
*/
#[derive(crate::Marker!)]
pub struct R;
