/**
# `\fq`

[Source](https://ubsicap.github.io/usfm/notes_basic/fnotes.html#fq)

- **Syntax**: `\fq_text...`
- **Type**: `character (note)`
- **Added**: `1.0`
- **Use**: Footnote translation quotation. \
    A quotation from the current scripture text translation for which the note is being provided. \
    Longer quotations are sometimes shortened using an ellipsis (i.e. suspension dots "...").

## Note

> Many existing translation texts have marked both quotations from the existing translation text, as well as alternative translations, using \fq. An additional marker – `\\fqa` – is provided for marking alternative translations, and can be used to distinguish between quotations and alternatives.

**Text and Formatting Samples - Quotations and Alternative Translations** - Mark 1.1; 1.4 (GNT)

```usfm
\s1 The Preaching of John the Baptist
\r (Matthew 3.1-12; Luke 3.1-18; John 1.19-28)
\p
\v 1 This is the Good News about Jesus Christ, the Son of God. \f + \fr 1.1: \ft Some
manuscripts do not have \fq the Son of God.\f*
...
```
*/
#[derive(crate::Marker!)]
pub struct FQ;
