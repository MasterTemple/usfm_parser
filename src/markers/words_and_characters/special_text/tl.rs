/**
# `\tl ...\tl\*`

[Source](https://ubsicap.github.io/usfm/characters/index.html#tl-tl)

- **Syntax**: `\tl_text...\tl*`
- **Type**: `character`
- **Added**: `1.0`
- **Use**: Transliterated (or foreign) word(s).

**Text and Formatting Sample** - Matthew 27.46 (GNT)

```usfm
\s1 The Death of Jesus
\r (Mark 15.33-41; Luke 23.44-49; John 19.28-30)
\p
\v 45 At noon the whole country was covered with darkness, which lasted for three hours.
\v 46 At about three o'clock Jesus cried out with a loud shout, \tl “Eli, Eli, lema
sabachthani?”\tl* which means, “My God, my God, why did you abandon me?”
```
*/
#[derive(crate::Marker!)]
pub struct TL;
