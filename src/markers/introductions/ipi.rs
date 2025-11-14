/**
# `\ipi`

[Source](https://ubsicap.github.io/usfm/introductions/index.html#ipi)

- **Syntax**: `\ipi_text...`
- **Type**: `paragraph`
- **Added**: `1.0`
- **Use**: Indented introduction paragraph. \

**Text and Formatting Sample** - Introduction to the Deuterocanonicals/Apocrypha (GCEV)

```usfm
\ip The following lists summarize each Christian tradition’s views of the books here designated
as Deuterocanonicals/Apocrypha.
\ipi Many Protestants consider the following books to be Apocrypha as defined above: Tobit,
Judith, additions to Esther (as found in Greek Esther in the CEV) ...
\ipi Roman Catholics consider the following books to be Deuterocanonical and of equal status
with all other books of the Old Testament: Tobit, Judith, Greek Esther ...
```
*/
#[derive(crate::Marker!)]
pub struct IPI;
