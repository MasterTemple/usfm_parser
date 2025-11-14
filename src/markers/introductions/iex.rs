/**
# `\iex`

[Source](https://ubsicap.github.io/usfm/introductions/index.html#iex)

- **Syntax**: `\iex_text...`
- **Type**: `paragraph`
- **Added**: `1.0`
- **Use**: Introduction explanatory or bridge text. \ \
    *Recommended use:* is for an explanation of missing book or section in a short Old Testament, or for attribution sentences found at the end of the 14 Pauline Epistles (most often found in hand written texts to identify the author, place of composition but does occur in some printed works).

**Text Sample** - After Romans 16 (KJV54 - BFBS)

```usfm
\v 27 to God only wise, \add be\add* glory through Jesus Christ for ever. Amen.
\iex Written to the Romans from Corinthus, and sent by Phebe servant of the church at Cenchrea.
```
*/
#[derive(crate::Marker!)]
pub struct IEX;
crate::impl_simple_tag!(IEX, "iex");
