/**
# `\io#`

[Source](https://ubsicap.github.io/usfm/introductions/index.html#io)

- **Syntax**: `\io#_text...(references range)`
- **Type**: `paragraph`
- **Added**: `1.0`
- **Use**: Introduction outline entry. \
    The outline entry typically ends with a range of references in parentheses. References may be marked with `\ior ...\ior*`. \
    The variable # represents the outline (indent) level. \
    **\io = \io1** (see `syntax notes` on numbered markers)

**Text and Formatting Sample** - Introduction to Mark (CEV)

```usfm
\ip The two endings to the Gospel, which are enclosed in brackets, are generally regarded
as written by someone other than the author of \bk Mark\bk*
\iot Outline of Contents
\io1 The beginning of the gospel (1.1-13)
\io1 Jesus' public ministry in Galilee (1.14–9.50)
\io1 From Galilee to Jerusalem (10.1-52)
\io1 The last week in and near Jerusalem (11.1–15.47)
\io1 The resurrection of Jesus (16.1-8)
\io1 The appearances and ascension of the risen Lord (16.9-20)
\c 1
\s The Preaching of John the Baptist
\r (Matthew 3.1-12; Luke 3.1-18; John 1.19-28)
\p
\v 1 This is the Good News about Jesus Christ
```
*/
#[derive(crate::Marker!)]
pub struct IO;
crate::impl_weighted_tag!(IO, "io");
