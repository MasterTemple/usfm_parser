/**
# `\qt ...\\qt\*`

[Source](https://ubsicap.github.io/usfm/characters/index.html#qt-qt)

- **Syntax**: `\qt_text...\qt*`
- **Type**: `character`
- **Added**: `1.0`
- **Use**: Quoted text. \
    Old Testament quotations in the New Testament, or other quotations.

**Text and Formatting Samples**

Poetic format, where all text is a quotation.

```usfm
\q1 \qt ........................................\qt*
\q2 \qt ........................................\qt*
\q1 \qt ........................................\qt*
\q2 \qt ........................................\qt*
```

Poetic format, where text is mixed (only some is a quotation).

```usfm
\q1 \qt ...............\qt* ..............\qt ........\qt*
\q2 \qt .....\qt* ................................
\q1 ..................................................
\q2 \qt.........................................\qt*
```

Mark 1.2-3 (GNT)

```usfm
\p
\v 1 This is the Good News about Jesus Christ, the Son of God.
\v 2 It began as the prophet Isaiah had written:
\q1 \qt “God said, ‘I will send my messenger ahead of you\qt*
\q2 \qt to open the way for you.’\qt*
\q1
\v 3 \qt Someone is shouting in the desert,\qt*
\q2 \qt ‘Get the road ready for the Lord;\qt*
\q2 \qt make a straight path for him to travel!’ ”\qt*
```
*/
#[derive(crate::Marker!)]
pub struct QT;
