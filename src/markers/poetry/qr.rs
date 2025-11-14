/**
# `\qr`

[Source](https://ubsicap.github.io/usfm/poetry/index.html#qr)

- **Syntax**: `\qr_text...`
- **Type**: `paragraph`
- **Added**: `1.0`
- **Use**: Right-aligned poetic line. \
    Commonly used for a poetic refrain.

**Text and Formatting Sample** - Psalm 136.1-3 (CEV - *markup adapted*)

```usfm
\c 136
\s1 God's Love Never Fails
\q1
\v 1 Praise the \nd Lord\nd*! He is good.
\qr God's love never fails.
\q1
\v 2 Praise the God of all gods.
\qr God's love never fails.
\q1
\v 3 Praise the Lord of lords.
\qr God's love never fails.
```
*/
#[derive(crate::Marker!)]
pub struct QR;
crate::impl_simple_tag!(QR, "qr");
