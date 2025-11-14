/**
# `\b`

[Source](https://ubsicap.github.io/usfm/poetry/index.html#b)

- **Syntax**: `\b`
- **Type**: `paragraph`
- **Added**: `1.0`
- **Use**: Blank line. \
    Use for stanza breaks in poetry, or between poetry and prose.

## Warning

> No text should follow this marker, and it should not be used before or after titles to indicate white-space.

**Text and Formatting Samples** - Psalm 3 (GNT)

```usfm
\c 3
\s1 Morning Prayer for Help
\q1
\v 1 I have so many enemies, \nd Lord\nd*,
\q2 so many who turn against me!
\q1
\v 2 They talk about me and say,
\q2 “God will not help him.”
\b
\q1
\v 3 But you, O \nd Lord\nd*, are always my shield from danger;
\q2 you give me victory
\q2 and restore my courage.
\q1
\v 4 I call to the \nd Lord\nd* for help,
\q2 and from his sacred hill he answers me.
\b
\q1
\v 5 I lie down and sleep,
\q2 and all night long the \nd Lord\nd* protects me.
\q1
\v 6 I am not afraid of the thousands of enemies
\q2 who surround me on every side.
```

Habakkuk 3.1 (GNT)

```usfm
\c 3
\s1 A Prayer of Habakkuk
\p
\v 1 This is a prayer of the prophet Habakkuk:
\b
\q1
\v 2 O \nd Lord\nd*, I have heard of what you have done,
\q2 and I am filled with awe.
```
*/
pub struct B;
