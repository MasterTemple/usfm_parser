/**
# `\im`

[Source](https://ubsicap.github.io/usfm/introductions/index.html#im)

- **Syntax**: `\im_text...`
- **Type**: `paragraph`
- **Added**: `1.0`
- **Use**: Introduction flush left (margin) paragraph. \

**Text and Formatting Sample** - Introduction to the GCEV

```usfm
\imt1 Preface:
\is1 A Word about the Contemporary English Version
\imi \em Translation it is that opens the window, to let in the light; that breaks the shell,
that we may eat the kernel; that puts aside the curtain, that we may look into the most holy place;
that removes the cover of the well, that we may come by the water.\em* (“The Translators to the
Reader,” King James Version, 1611).
\im The most important document in the history of the English language is the \bk King James
Version\bk* of the Bible...
```

*/
#[derive(crate::Marker!)]
pub struct IM;
crate::impl_simple_tag!(IM, "im");
