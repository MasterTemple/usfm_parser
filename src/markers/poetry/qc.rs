/**
# `\qc`

[Source](https://ubsicap.github.io/usfm/poetry/index.html#qc)

- **Syntax**: `\qc_text...`
- **Type**: `paragraph`
- **Added**: `1.0`
- **Use**: Centered poetic line.

**Text and Formatting Sample** - Psalm 72.19 (GNT)

```usfm
\q1
\v 18 Praise the \nd Lord\nd*, the God of Israel!
\q1 He alone does these wonderful things.
\q1
\v 19 Praise his glorious name forever!
\q1 May his glory fill the whole world.
\b
\qc Amen! Amen!
\b
\q1
\v 20 This is the end of the prayers of David son of Jesse.
```
*/
#[derive(crate::Marker!)]
pub struct QC;
crate::impl_simple_tag!(QC, "qc");
