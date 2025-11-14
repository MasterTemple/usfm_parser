/**
# `\imt#`

[Source](https://ubsicap.github.io/usfm/introductions/index.html#imt)

- **Syntax**: `\imt#_text...`
- **Type**: `paragraph`
- **Added**: `1.0`
- **Use**: Introduction major title. \
    The variable # denotes the title level or relative weighting. \
    **\imt = \imt1** (see `syntax notes` on numbered markers) \ \
    *Recommended use* is for the introduction title or other major introduction division (rather than `\is`) when the introduction text contains numerous additional sub-divisions.

**Text and Formatting Sample** - Introduction to Mark (RVE)

```usfm
\h SAN MARCOS
\mt2 Evangelio según
\mt1 SAN MARCOS
\imt1 INTRODUCCIÓN
\is1 Importancia del evangelio de Marcos
\ip Este evangelio, segundo de los libros del NT, contiene poco material que no aparezca
igualmente en \bk Mateo\bk* y \bk Lucas.\bk*
```
*/
#[derive(crate::Marker!, crate::Weighted!)]
pub struct IMT(Option<u8>);
crate::impl_weighted_tag!(IMT, "imt");
