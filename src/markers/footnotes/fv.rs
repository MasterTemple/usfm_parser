/**
# `\fv ...\fv*`

[Source](https://ubsicap.github.io/usfm/notes_basic/fnotes.html#fv-fv)

- **Syntax**: `\fv_##\fv*`
- **Type**: `character (note)`
- **Added**: `1.0`
- **Use**: Footnote verse number. \
    A verse number in the the text quotation or alternative translation.

## Note

> This marker will typically be nested within another footnote content element, like `\ft`, `\fq` or `\fqa`. See `Nesting Character Markup` for details.

**Text and Formatting Sample** - John 7.38 (GNT)

```usfm
\p
\v 37 On the last and most important day of the festival Jesus stood up and said in a
loud voice, “Whoever is thirsty should come to me, and
\v 38 whoever believes in me should drink. As the scripture says, ‘Streams of life-
giving water will pour out from his side.’” \f + \fr 7.38: \ft Jesus' words in verses
37-38 may be translated: \fqa “Whoever is thirsty should come to me and drink.
\+fv 38\+fv* As the scripture says, ‘Streams of life-giving water will pour out from
within anyone who believes in me.’”\f*
```
*/
#[derive(crate::Marker!)]
pub struct FV;
