/**
# `\pc`

[Source](https://ubsicap.github.io/usfm/paragraphs/index.html#pc)

- **Syntax**: `\pc(_text...)`
- **Type**: `paragraph`
- **Added**: `1.0`
- **Use**: Centered paragraph. \
    *Recommended use:* Inscriptions.

**Text and Formatting Sample** - Revelation 17.5 (CEV)

```usfm
\v 4 The woman was dressed in purple and scarlet robes, and she wore jewelry made of
gold, precious stones, and pearls. In her hand she held a gold cup filled with the filthy
and nasty things she had done.
\v 5 On her forehead a mysterious name was written:
\pc I AM THE GREAT CITY OF BABYLON, THE MOTHER OF EVERY IMMORAL AND FILTHY THING ON EARTH.
\m
\v 6 I could tell that the woman was drunk on the blood of God's people who had given
their lives for Jesus. This surprising sight amazed me, ...
```
*/
#[derive(crate::Marker!)]
pub struct PC;
crate::impl_simple_tag!(PC, "pc");
