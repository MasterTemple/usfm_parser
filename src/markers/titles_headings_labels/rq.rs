/**
# `\rq ...\\rq\*`

[Source](https://ubsicap.github.io/usfm/titles_headings/index.html#rq-rq)

- **Syntax**: `\rq ...\rq*`
- **Type**: `character`
- **Added**: `1.0`
- **Use**: Inline quotation reference(s). \
    A reference indicating the source text for the preceding quotation (usually an Old Testament quote).

.. note::
    \\rq ...\\rq* reference(s) are intended to be formatted (typeset) within the scripture body text column, and not extracted from the text as are regular cross references (`\\x ...\\x*`). They are also typically separated from the main text of Scripture using a different type style and alignment.

**Text and Formatting Sample** - Hebrews 1.5 (GNT)

```usfm
\p
\v 4 The Son was made greater than the angels, just as the name that God gave him is greater
than theirs.
\v 5 For God never said to any of his angels,
\q1 "You are my Son;
\q2 today I have become your Father."
\rq Psa 2.7\rq*
\b
\m Nor did God say about any angel,
\q1 "I will be his Father,
\q2 and he will be my Son."
\rq 2Sa 7.14; 1Ch 17.13\rq*
```
*/
pub struct RQ;
