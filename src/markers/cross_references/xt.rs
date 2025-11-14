/**
# `\xt`

[Source](https://ubsicap.github.io/usfm/notes_basic/xrefs.html#xt)

- **Syntax**: `\\xt_refs...`
- **Type**: `character (note)`
- **Added**: `1.0`
- **Updated**: 3.0 (attributes)
- **Use**: Target reference(s). \
    A list of scripture references, commonly provided as book name abbreviations plus chapter and verse, or range of verses. The punctuation used between chapter and verse, reference ranges, and between target references can differ significantly across texts.

**Text and Formatting Samples - Typical Cross Reference** - Matthew 2.23 (GNT)

```usfm
\p
\v 22 But when Joseph heard that Archelaus had succeeded his father Herod as king of
Judea, he was afraid to go there. He was given more instructions in a dream, so he went
to the province of Galilee
\v 23 \x - \xo 2.23: \xt Mrk 1.24; Luk 2.39; Jhn 1.45.\x* and made his home in a town
named Nazareth. And so what the prophets had said came true: “He will be called a
Nazarene.”
```

**Multiple Origin Parts** - Mark 10.19 (GNT)

```usfm
\p
\v 18 “Why do you call me good?” Jesus asked him. “No one is good except God alone.
\v 19 \x - \xo 10.19: a \xt Exo 20.13; Deu 5.17; \xo b \xt Exo 20.14; Deu 5.18; \xo c
\xt Exo 20.15; Deu 5.19; \xo d \xt Exo 20.16; Deu 5.20; \xo e \xt Exo 20.12; Deu 5.16.\x*
You know the commandments: ‘Do not commit murder; do not commit adultery; do not steal;
do not accuse anyone falsely; do not cheat; respect your father and your mother.’”
```
*/
#[derive(crate::Marker!)]
pub struct XT;
