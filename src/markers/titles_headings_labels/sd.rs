/**
# `\sd#`

[Source](https://ubsicap.github.io/usfm/titles_headings/index.html#sd)

- **Syntax**: `\sd#`
- **Type**: `paragraph`
- **Added**: `3.0`
- **Use**: Semantic division (semantic space). \
    Vertical space used to divide the text into sections, in a manner similar to the structure added through the use of a sequence of heading texts (i.e. `\\ms#` and `\\s#`). \
    The purpose of `\sd#` is distinct from `\\b` which primarily denotes whitespace (and in particular at poetic stanza breaks) and not hierarchy or division.
    The variable # represents the level of division being marked. \
    **\\sd = \\sd1** (see `syntax notes` on numbered markers)

**Text and Formatting Sample** - Matthew 13.51-54 (NIV "Books of the Bible"; chapter and verse numbers suppressed in layout; new sections begin with drop capital)

```usfm
\m
\v 51 “Have you understood all these things?” Jesus asked.
\p “Yes,” they replied.
\p
\v 52 He said to them, “Therefore every teacher of the law who has been instructed about
the kingdom of heaven is like the owner of a house who brings out of his storeroom new
treasures as well as old.”
\sd2
\p
\v 53 When Jesus had finished these parables, he moved on from there.
\v 54 Coming to his hometown, he began teaching the people in their synagogue, and they
were amazed. “Where did this man get this wisdom and these miraculous powers?” they asked.
```
*/
pub struct SD;
