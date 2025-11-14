/**
# `\cl`

[Source](https://ubsicap.github.io/usfm/chapters_verses/index.html#cl)

- **Syntax**: `\cl_text...`
- **Type**: `paragraph`
- **Added**: `1.0`
- **Use**: The chapter "label" to be used when the chosen publishing presentation will render chapter divisions as headings (not drop cap numerals).

.. note::

    **Usage note:** If \\cl is entered once before chapter 1 (\\c 1) it represents the text for "chapter" to be used throughout the current book. If \\cl is used after each individual chapter marker, it represents the particular text to be used for the display of the current chapter heading (usually done if numbers are being presented as words, not numerals).

**Text and Formatting Samples** - Psalm 1 (GNT - *markup adapted* - general chapter label)

```usfm
\cl Psalm
\c 1
\q1
\v 1 Happy are those
\q2 who reject the advice of evil people,
```
*/
pub struct CL;
