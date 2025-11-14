/**
# `\fdc ...\fdc*`

[Source](https://ubsicap.github.io/usfm/notes_basic/fnotes.html#fdc-fdc)

- **Syntax**: `\fdc_text...\fdc*`
- **Type**: `character (note)`
- **Added**: `1.0`
- **Deprecated**: 3.0 :Use: Footnote Deuterocanonical content. \
    Text between these markers is material to be included only in published editions that contain the Deuterocanonical books. \
    **Deprecated** (use is discouraged). \ \
    |ico_Cg| *Recommended alternate:* General purpose use of `\dc ...\dc*` or `nested` `\+dc ...\+dc*` wherever DC-only content is being marked.

**Text and Formatting Sample** - Hebrews 1.3 (Spanish DHE)

```usfm
\v 3 Él es el resplandor glorioso de Dios,\f c \fr 1.3: \fk Resplandor: \ft Cf.
Jn 1.4-9,14\fdc ; también Sab 7.25-26, donde algo parecido se dice de la sabiduría.\f*
la imagen misma de lo que Dios es y el que sostiene todas las cosas con su palabra
poderosa. Después de limpiarnos de nuestros pecados, se ha sentado en el cielo, a la
derecha del trono de Dios,
\v 4 y ha llegado a ser superior a los ángeles, pues ha recibido en herencia un título
mucho más importante que el de ellos.
```
*/
#[derive(crate::Marker!)]
pub struct FDC;
crate::impl_paired_tag!(FDC, "fdc");
