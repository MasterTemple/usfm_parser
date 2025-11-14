/**
# `\qac ...\qac*`

[Source](https://ubsicap.github.io/usfm/poetry/index.html#qac-qac)

- **Syntax**: `\qac_text...\qac*`
- **Type**: `character`
- **Added**: `1.0`
- **Use**: Marker to indicate the acrostic letter within a poetic line. \
    *A character style.*

**Text and Formatting Sample** - Lamentations 1.1,2 (Spanish TLA)

```usfm
\c 1
\s1 Primer lamento acróstico
\s2 El profeta
\q1
\v 1 ¡\qac P\qac*obrecita de ti, Jerusalén!
\q1 Antes eras la más famosa
\q1 de todas las ciudades.
\q1 ¡Antes estabas llena de gente,
\q1 pero te has quedado muy sola,
\q1 te has quedado viuda!
\q1 ¡Fuiste la reina de las naciones,
\q1 pero hoy eres esclava de ellas!
\b
\q1
\v 2 \qac O\qac*lvidada y bañada en lágrimas
\q1 pasas todas las noches.
\q1 Muchos decían que te amaban,
\q1 pero hoy nadie te consuela.
\q1 Los que se decían tus amigos
\q1 hoy son tus enemigos.
```
*/
#[derive(crate::Marker!)]
pub struct QAC;
crate::impl_paired_tag!(QAC, "qac");
