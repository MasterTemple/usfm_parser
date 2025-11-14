/**
# `\p`

[Source](https://ubsicap.github.io/usfm/paragraphs/index.html#p)

- **Syntax**: `\p(_text...)`
- **Type**: `paragraph`
- **Added**: `1.0`
- **Use**: Normal paragraph. \
    Followed immediately by a space and paragraph text, or by a new line and a verse marker.

**Text and Formatting Sample** - Mark 1.1-4 (GNT)

```usfm
\c 1
\s1 The Preaching of John the Baptist
\r (Matthew 3.1-12; Luke 3.1-18; John 1.19-28)
\p
\v 1 This is the Good News about Jesus Christ, the Son of God.
\v 2 It began as the prophet Isaiah had written:
\q1 “God said, ‘I will send my messenger ahead of you
\q2 to open the way for you.’
\q1
\v 3 Someone is shouting in the desert,
\q2 ‘Get the road ready for the Lord;
\q2 make a straight path for him to travel!’”
\p
\v 4 So John appeared in the desert, baptizing and preaching. “Turn away from your sins
and be baptized,” he told the people, “and God will forgive your sins.”
```
*/
pub struct P;
