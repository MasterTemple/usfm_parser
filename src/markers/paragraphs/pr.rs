/**
# `\pr`

[Source](https://ubsicap.github.io/usfm/paragraphs/index.html#pr)

- **Syntax**: `\pr(_text...)`
- **Type**: `paragraph`
- **Added**: `1.0`
- **Deprecated**: 2.0
- **Restored**: 3.0 :Use: Right-aligned paragraph. \
    *Recommended use:* Text refrain.

**Text and Formatting Sample** - Deuteronomy 27.15,16,17 (GNT - *markup adapted*)

```usfm
\p
\v 15 “ ‘God's curse on anyone who makes an idol of stone, wood, or metal and secretly
worships it; the \nd Lord\nd* hates idolatry.’
\pr “And all the people will answer, ‘Amen!’
\p
\v 16 “ ‘God's curse on anyone who dishonors his father or mother.’
\pr “And all the people will answer, ‘Amen!’
\p
\v 17 “ ‘God's curse on anyone who moves a neighbor's property line.’
\pr “And all the people will answer, ‘Amen!’
```
*/
pub struct PR;
