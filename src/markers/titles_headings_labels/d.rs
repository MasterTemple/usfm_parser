/**
# `\d`

[Source](https://ubsicap.github.io/usfm/titles_headings/index.html#d)

- **Syntax**: `\d_text...`
- **Type**: `paragraph`
- **Added**: `1.0`
- **Use**: Descriptive title (or "Hebrew subtitle"). \
    Sometimes used in Psalms under the section heading (e.g. "For the director of Music").

**Text and Formatting Sample** - Psalm 3.1 (NRSV)

```usfm
\c 3
\s1 Trust in God under Adversity
\d A Psalm of David, when he fled from his son Absalom.
\q1
\v 1 O \nd Lord\nd*, how many are my foes!
\q2 Many are rising against me;
\q1
\v 2 many are saying to me,
\q2 “There is no help for you in God.” \qs Selah\qs*
```
*/
#[derive(crate::Marker!)]
pub struct D;
