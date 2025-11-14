/**
# `\id`

[Source](https://ubsicap.github.io/usfm/identification/index.html#index-1)

- **Syntax**: `\id_<CODE>_(Name of file, Book name, Language, Last edited, Date etc.)`
- **Type**: `paragraph`
- **Added**: `1.0`
- **Use**: File identification. \
    This is the initial USFM marker in any scripture text file. \
    CODE is a standard 3 letter `scripture book abbreviation`.

**Text and Formatting Sample**

```usfm
\id MAT 41MATGNT92.SFM, Good News Translation, June 2003
```

The text following this marker is not normally used in any formatted presentation.
*/
#[derive(crate::Marker!)]
pub struct ID;
crate::impl_simple_tag!(ID, "id");
