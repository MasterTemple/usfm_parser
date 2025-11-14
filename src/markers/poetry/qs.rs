/**
# `\qs ...\qs\*`

[Source](https://ubsicap.github.io/usfm/poetry/index.html#qs-qs)

- **Syntax**: `\qs_(Selah)\qs*`
- **Type**: `character`
- **Added**: `1.0`
- **Use**: Used for the expression "Selah" commonly found in Psalms and Habakkuk. \
    *A character style.* \
    This text is frequently right aligned, and rendered on the same line as the previous poetic text, if space allows.

**Text and Formatting Samples** - Psalm 3.2 (NRSV)

```usfm
\c 3
\s1 Trust in God under Adversity
\d A Psalm of David, when he fled from his son Absalom.
\q1
\v 1 O \nd Lord\nd*, how many are my foes!
\q2 Many are rising against me;
\q1
\v 2 many are saying to me,
\q2 “There is no help for you \qs Selah\qs*
```

Psalm 24.10 (NRSV)

```usfm
\q1
\v 10 Who is this King of glory?
\q2 The \nd Lord\nd* of hosts,
\q2 he is the King of glory. \qs Selah\qs*
```
*/
#[derive(crate::Marker!)]
pub struct QS;
