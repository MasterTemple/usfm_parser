/**
# `\pi#`

[Source](https://ubsicap.github.io/usfm/paragraphs/index.html#pi)

- **Syntax**: `\pi#(_Sample text...)`
- **Type**: `paragraph`
- **Added**: `1.0`
- **Use**: Indented paragraph. \
    Used in some texts for discourse sections. \
    The variable # represents the level of indent. \
    *See also* `\pm` \
    **\pi = \pi1** (see `syntax notes` on numbered markers)

**Text and Formatting Sample** - Matthew 13.37-39 (CEV)

```usfm
\s1 Jesus Explains the Story about the Weeds
\p
\v 36 After Jesus left the crowd and went inside, his disciples came to him and said,
“Explain to us the story about the weeds in the wheat field.”
\p
\v 37 Jesus answered:
\pi The one who scattered the good seed is the Son of Man.
\v 38 The field is the world, and the good seeds are the people who belong to the kingdom.
The weed seeds are those who belong to the evil one,
\v 39 and the one who scattered them is the devil. The harvest is the end of time, and
angels are the ones who bring in the harvest.
```
*/
#[derive(crate::Marker!, crate::Weighted!)]
pub struct PI(Option<u8>);
crate::impl_weighted_tag!(PI, "pi");
