/**
# `\xdc ...\xdc*`

[Source](https://ubsicap.github.io/usfm/notes_basic/xrefs.html#xdc-xdc)

- **Syntax**: `\xdc_refs...\xdc*`
- **Type**: `character (note)`
- **Added**: `1.0`
- **Deprecated**: 3.0 :Use: References (or other text) between these markers is material to be included only in published editions that contain the Deuterocanonical books. \
    **Deprecated** (use is discouraged). \
    *Recommended alternate:* General purpose use of `\dc ...\dc*` or `nested` `\+dc ...\+dc*` wherever DC-only content is being marked.

**Text and Formatting Samples** - Psalm 115.3-4 (GNT - cross references)

```usfm
\q1
\v 3 Our God is in heaven;
\q2 he does whatever he wishes.
\q1
\v 4 \x - \xo 115.4-8: \xt Ps 135.15-18; \xdc Ltj Jr 4-73; \xt Rev 9.20.\x* Their
gods are made of silver and gold,
\q2 formed by human hands.
```

1 Corinthians 15.51-52 (GNT - cross reference)

```usfm
\p
\v 51-52 \x - \xo 15.51,52: \xdc 2Es 6.23; \xt 1Th 4.15-17.\x* Listen to this secret
truth: we shall not all die, but when the last trumpet sounds, we shall all be changed
in an instant, as quickly as the blinking of an eye. For when the trumpet sounds, the
dead will be raised, never to die again, and we shall all be changed.
```
*/
#[derive(crate::Marker!)]
pub struct XDC;
