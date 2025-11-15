/**
# `\nd ...\nd*`

[Source](https://ubsicap.github.io/usfm/characters/index.html#nd-nd)

- **Syntax**: `\nd_text...\nd*`
- **Type**: `character`
- **Added**: `1.0`
- **Use**:  Name of God (name of Deity).

**Text and Formatting Sample** - Exodus 3.15 (GNT)

```usfm
\p
\v 14 God said, “I am who I am. You must tell them: ‘The one who is called I AM  has
sent me to you.’
\v 15 Tell the Israelites that I, the \nd Lord\nd*, the God of their ancestors, the God
of Abraham, Isaac, and Jacob, have sent you to them. This is my name forever; this is
what all future generations are to call me.
```

*/
#[derive(crate::Marker!, crate::Deref!)]
pub struct ND(bool);
crate::impl_paired_tag!(ND, "nd");
