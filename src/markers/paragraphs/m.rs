/**
# `\m`

[Source](https://ubsicap.github.io/usfm/paragraphs/index.html#m)

- **Syntax**: `\m(_text...)`
- **Type**: `paragraph`
- **Added**: `1.0`
- **Use**: Continuation (margin) paragraph. \
    No first line indent. \
    Followed immediately by a space and paragraph text, or by a new line and a verse marker. \
    Usually used to resume prose at the margin (without indent) after poetry or OT quotation (i.e. continuation of the previous paragraph).

**Text and Formatting Sample** - Mark 12.37 (GNT)

```usfm
\p
\v 35 As Jesus was teaching in the Temple, he asked the question, “How can the teachers
of the Law say that the Messiah will be the descendant of David?
\v 36 The Holy Spirit inspired David to say:
\q1 ‘The Lord said to my Lord:
\q2 Sit here at my right side
\q2 until I put your enemies under your feet.’
\b
\m
\v 37 David himself called him ‘Lord’; so how can the Messiah be David's descendant?”
```
*/
#[derive(crate::Marker!)]
pub struct M;
crate::impl_simple_tag!(M, "m");
