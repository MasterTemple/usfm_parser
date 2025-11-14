/**
# `\dc ...\\dc\*`

[Source](https://ubsicap.github.io/usfm/characters/index.html#dc-dc)

- **Syntax**: `\dc_text...\dc*`
- **Type**: `character`
- **Added**: `1.0`
- **Use**: Deuterocanonical/LXX additions or insertions in the Protocanonical text. \
    General purpose use of `\dc ...\dc*` or `nested` `\+dc ...\+dc*` is encouraged wherever DC-only content is being marked. Use of context-specific DC-only markup (i.e. `\\fdc ...\\fdc\*`, `\\xdc ...\\xdc\*`) is discouraged.

**Text and Formatting Samples** - Hebrews 1.3 (Spanish DHE - footnote)

```usfm
\v 3 Él es el resplandor glorioso de Dios,\f c \fr 1.3: \fk Resplandor: \ft Cf.
Jn 1.4-9,14\+dc ; también Sab 7.25-26, donde algo parecido se dice de la sabiduría\+dc*.\f*
la imagen misma de lo que Dios es y el que sostiene todas las cosas con su palabra
poderosa. Después de limpiarnos de nuestros pecados, se ha sentado en el cielo, a la
derecha del trono de Dios,
\v 4 y ha llegado a ser superior a los ángeles, pues ha recibido en herencia un título
mucho más importante que el de ellos.
```

Psalm 115.3-4 (GNT - cross references)

```usfm
\q1
\v 3 Our God is in heaven;
\q2 he does whatever he wishes.
\q1
\v 4 \x - \xo 115.4-8: \xt Ps 135.15-18; \+dc Ltj Jr 4-73; \+dc*\xt Rev 9.20.\x* Their
gods are made of silver and gold,
\q2 formed by human hands.
```

1 Corinthians 15.51-52 (GNT - cross reference)

```usfm
\p
\v 51-52 \x - \xo 15.51,52: \xt \+dc 2Es 6.23; \+dc*1Th 4.15-17.\x* Listen to this secret
truth: we shall not all die, but when the last trumpet sounds, we shall all be changed
in an instant, as quickly as the blinking of an eye. For when the trumpet sounds, the
dead will be raised, never to die again, and we shall all be changed.
```
*/
#[derive(crate::Marker!)]
pub struct DC;
