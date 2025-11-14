/**
# `\rem`

[Source](https://ubsicap.github.io/usfm/identification/index.html#rem)

- **Syntax**: `\rem_text...`
- **Type**: `paragraph`
- **Added**: `1.0`
- **Use**: Remark. \
    For adding brief comments by a translator, consultant, or support person. The text is not a type of footnote and is not intended for publication. When `\rem` is used, it is often found at the top of a USFM file together with other identification material. However, the use of `\rem` is not limited to this section of a USFM file. It can be used for adding a paragraph containing non-publishable remarks / comments anywhere within a text.

**Text and Formatting Sample**

```usfm
\rem First draft complete, waiting for checks.
```

The text following this marker is not normally used in any formatted presentation.

## Warning

> Adding names of individuals, initials, or other personal information directly within scripture text files is *strongly discouraged*.
*/
pub struct REM;
