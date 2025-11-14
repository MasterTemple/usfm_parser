/**
# `\v`

[Source](https://ubsicap.github.io/usfm/chapters_verses/index.html#v)

- **Syntax**: `\v_#_`
- **Type**: `character`
- **Added**: `1.0`
- **Use**: Verse number. \
    The marker is followed by the verse number #, then a space (which marks the end of the verse), and then text of the verse.

**Text and Formatting Sample** - Matthew 1.18,19 (GNT)

```usfm
\s1 The Birth of Jesus Christ
\r (Luke 2.1-7)
\p
\v 18 This was how the birth of Jesus Christ took place. His mother Mary was engaged
to Joseph, but before they were married, she found out that she was going to have a
baby by the Holy Spirit.
\v 19 Joseph was a man who always did what was right, but he did not want to disgrace
Mary publicly; so he made plans to break the engagement privately.
```
*/
#[derive(crate::Marker!)]
pub struct V;
crate::impl_simple_tag!(V, "v");
