/**
# `\xta`

[Source](https://ubsicap.github.io/usfm/notes_basic/xrefs.html#xta)

- **Syntax**: `\\xta_text...`
- **Type**: `character (note)`
- **Added**: `3.0`
- **Use**: Target reference(s) extra / added text. \
    Used for marking text which should be ignored when identifying or linking to cross reference target references.

**Text and Formatting Sample** - Matthew 3.0 (GNT - *text and markup adapted*)

```usfm
\c 3
\s1 The Preaching of John the Baptist\x - \xo 3.0 \xta Compare with \xt Mk 1.1-8;
Lk 3.1-18; \xta and \xt Jn 1.19-28 \xta parallel passages.\x*
\p
\v 1 At that time John the Baptist came to the desert of Judea and started preaching.
```
*/
#[derive(crate::Marker!)]
pub struct XTA;
