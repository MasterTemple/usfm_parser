/**
# `\ide`

[Source](https://ubsicap.github.io/usfm/identification/index.html#index-3)

- **Syntax**: `\ide_<ENCODING>`
- **Type**: `paragraph`
- **Added**: `1.0`
- **Use**: An optional character encoding specification. \
    This marker should be used to specify the character encoding of the text within the file. For example: CP-1252, CP-1251, UTF-8, UTF-16, OR Custom <specify font name>. If the character encoding does not conform to a known standard, but is rather a customized solution for the project, a minimum of the name of the font used for the project should be included. For archive purposes, texts which rely upon a custom encoding solution should be converted to Unicode, if at all possible.

**Text and Formatting Sample**

```usfm
\ide UTF-8
\ide CP-1252
\ide Custom (TGUARANI.TTF)
```

The text following this marker is not normally used in any formatted presentation.
*/
#[derive(crate::Marker!)]
pub struct IDE;
