/**
# `\fk`

[Source](https://ubsicap.github.io/usfm/notes_basic/fnotes.html#fk)

- **Syntax**: `\fk_text...`
- **Type**: `character (note)`
- **Added**: `1.0`
- **Use**: Footnote keyword. \
    The specific keyword/term from the text for which the footnote is being provided.

**Text and Formatting Sample** - Genesis 3.20 (GNT)

```usfm
\p
\v 20 Adam \f + \fr 3.20: \fk Adam: \ft This name in Hebrew means “all human beings.”\f*
named his wife Eve, \f + \fr 3.20: \fk Eve: \ft This name sounds similar to the Hebrew
word for “living,” which is rendered in this context as “human beings.”\f* because she
was the mother of all human beings.
\v 21 And the \nd Lord\nd* God made clothes out of animal skins for Adam and his wife,
and he clothed them.
```
*/
#[derive(crate::Marker!)]
pub struct FK;
