/**
# `\qd`

[Source](https://ubsicap.github.io/usfm/poetry/index.html#qd)

- **Syntax**: `\qd_text...`
- **Type**: `paragraph`
- **Added**: `3.0`
- **Use**: Hebrew note. \
    A Hebrew musical performance comment similar in content to many of the Hebrew Psalm titles (`\d`), but placed at the end of the poetic section.

**Text and Formatting Sample** - Habakkuk 3:19 (NIV)

```usfm
\q1 \v 18 yet I will rejoice in the \nd Lord\nd*,
\q2 I will be joyful in God my Savior.
\b
\q1 \v 19 The Sovereign \nd Lord\nd* is my strength;
\q2 he makes my feet like the feet of a deer,
\q2 he enables me to tread on the heights.
\b
\qd For the director of music. On my stringed instruments.
```
*/
#[derive(crate::Marker!)]
pub struct QD;
crate::impl_simple_tag!(QD, "qd");
