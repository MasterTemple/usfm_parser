/**
# `\q#`

[Source](https://ubsicap.github.io/usfm/poetry/index.html#b)

- **Syntax**: `\q#(_text...)`
- **Type**: `paragraph`
- **Added**: `1.0`
- **Use**: Poetic line. \
    The variable # represents the level of indent (i.e. \q1, \q2, \q3 etc.). \
    **\q = \q1** (see `syntax notes` on numbered markers)

**Text and Formatting Examples** - Habakkuk 3.1-2 (GNT)

```usfm
\c 3
\s1 A Prayer of Habakkuk
\p
\v 1 This is a prayer of the prophet Habakkuk:
\b
\q1
\v 2 O \nd Lord\nd*, I have heard of what you have done,
\q2 and I am filled with awe.
\q1 Now do again in our times
\q2 the great deeds you used to do.
\q1 Be merciful, even when you are angry.
```

```usfm
\q1
\v 2 O \nd Lord\nd*, I have heard of what you have done,
\q2 and I am filled with awe.
\q1 Now do again in our times
\q2 the great deeds you used to do.
\q1 Be merciful, even when you are angry.
```
*/
#[derive(crate::Marker!)]
pub struct Q;
