/**
# `\ior ...\\ior\*`

[Source](https://ubsicap.github.io/usfm/introductions/index.html#ior-ior)

- **Syntax**: `\ior_text...\ior*`
- **Type**: `character`
- **Added**: `1.0`
- **Use**: Introduction outline reference range. \
    An outline entry typically ends with a range of references in parentheses. This is an optional character style for marking (and potentially formatting) these references separately.

**Text and Formatting Sample** - Introduction to Mark (CEV)

```usfm
\io1 The beginning of the gospel \ior (1.1-13)\ior*
\io1 Jesus' public ministry in Galilee \ior (1.14–9.50)\ior*
\io1 From Galilee to Jerusalem \ior (10.1-52)\ior*
\io1 The last week in and near Jerusalem \ior (11.1–15.47)\ior*
\io1 The resurrection of Jesus \ior (16.1-8)\ior*
\io1 The appearances and ascension of the risen Lord \ior (16.9-20)\ior*
```
*/
#[derive(crate::Marker!)]
pub struct IOR;
