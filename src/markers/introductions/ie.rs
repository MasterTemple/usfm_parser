/**
# `\ie`

[Source](https://ubsicap.github.io/usfm/introductions/index.html#ie)

- **Syntax**: `\ie`
- **Type**: `paragraph`
- **Added**: `1.0`
- **Use**: Introduction end. \
    Optionally included to explicitly indicate the end of the introduction material.

**Text and Formatting Sample** - Introduction to Mark (GNT)

```usfm
\io1 The resurrection of Jesus (16.1-8)
\io1 The appearances and ascension of the risen Lord (16.9-20)
\ie
\c 1
\s The Preaching of John the Baptist
\r (Matthew 3.1-12; Luke 3.1-18; John 1.19-28)
\p
\v 1 This is the Good News about Jesus Christ
```
*/
#[derive(crate::Marker!)]
pub struct IE;
