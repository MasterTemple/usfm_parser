/**
# `\is#`

[Source](https://ubsicap.github.io/usfm/introductions/index.html#is)

- **Syntax**: `\is#_text...`
- **Type**: `paragraph`
- **Added**: `1.0`
- **Use**: Introduction section heading. \
    The variable # denotes the title level or relative weighting. \
    **\\is = \\is1** (see `syntax notes` on numbered markers)

**Text and Formatting Sample** - Introduction to Mark (RVE)

```usfm
\h SAN MARCOS
\mt2 Evangelio según
\mt1 SAN MARCOS
\imt1 INTRODUCCIÓN
\is1 Importancia del evangelio de Marcos
\ip Este evangelio, segundo de los libros del NT, contiene poco material que no aparezca
igualmente en \bk Mateo\bk* y \bk Lucas\bk*.
```
*/
#[derive(crate::Marker!)]
pub struct IS;
