/**
# `\iq#`

[Source](https://ubsicap.github.io/usfm/introductions/index.html#iq)

- **Syntax**: `\iq#_text...`
- **Type**: `paragraph`
- **Added**: `1.0`
- **Use**:     Introduction poetic line. \
    The variable # represents the indent level (i.e. \iq1, \iq2, \iq3 etc.) \
    **\iq = \iq1** (see `syntax notes` on numbered markers)

**Text and Formatting Sample** - Introduction to Titus (CEV)

```usfm
\ip Paul also tells how we are saved:
\ib
\iq1 God our Savior showed us
\iq2 how good and kind he is.
\iq1 He saved us because
\iq2 of his mercy,
\iq1 and not because
\iq2 of any good things
\iq2 that we have done.
\ipr (3.4,5a)
```
*/
#[derive(crate::Marker!)]
pub struct IQ;
crate::impl_weighted_tag!(IQ, "iq");
