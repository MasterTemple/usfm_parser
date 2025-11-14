/**
# `\mi`

[Source](https://ubsicap.github.io/usfm/paragraphs/index.html#mi)

- **Syntax**: `\mi(_text...)`
- **Type**: `paragraph`
- **Added**: `1.0`
- **Use**: Indented flush left paragraph. \
    No first line indent. \
    *See also* `\pmo`, `\pmc`

**Text and Formatting Sample** - Matthew 11.18-19 (CEV)

```usfm
\pi
\v 16 You people are like children sitting in the market and shouting to each other,
\b
\q1
\v 17 “We played the flute,
\q2 but you would not dance!
\q1 We sang a funeral song,
\q2 but you would not mourn!”
\b
\mi
\v 18 John the Baptist did not go around eating and drinking, and you said, “That man has
a demon in him!”
\v 19 But the Son of Man goes around eating and drinking, and you say, “That man eats and
drinks too much! He is even a friend of tax collectors ...
```
*/
#[derive(crate::Marker!)]
pub struct MI;
crate::impl_simple_tag!(MI, "mi");
