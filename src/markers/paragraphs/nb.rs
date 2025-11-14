/**
# `\nb`

[Source](https://ubsicap.github.io/usfm/paragraphs/index.html#nb)

- **Syntax**: `\nb`
- **Type**: `paragraph`
- **Added**: `1.0`
- **Use**: Basic. \
    Indicates "no-break" with previous paragraph (regardless of previous paragraph type). \
    Commonly used in cases where the previous paragraph spans the chapter boundary.

**Text and Formatting Sample** - John 7.53–8.2 (CEV)

```usfm
\p
\v 52 Then they said, “Nicodemus, you must be from Galilee! Read the Scriptures, and you
will find that no prophet is to come from Galilee.”
\s1 A Woman Caught in Sin
\p
\v 53 Everyone else went home,
\c 8
\nb
\v 1 but Jesus walked out to the Mount of Olives.
\v 2 Then early the next morning he went to the temple. The people came to him, and he
sat down and started teaching them.
```

## Note

> **No-break markup within poetry:** Some translations have a publishing tradition of inserting a small amount of additional white-space at chapter boundaries. It is important in these texts to use the \nb marker within any specific poetic contexts where no visible break in the flow of the the text is intended at a particular chapter boundary.
*/
#[derive(crate::Marker!)]
pub struct NB;
