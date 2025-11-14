/**
# `\usfm`

[Source](https://ubsicap.github.io/usfm/identification/index.html#usfm)

- **Syntax**: `\usfm_<USFM version number>`
- **Type**: `paragraph`
- **Added**: `3.0`
- **Use**: USFM version specification for the file. \
    Used to identify the USFM version which a USFM editor / processor will be required to support in order to manage all markup found within the file.

**Text Sample**

```usfm
\id MAT 41MATGNT92.SFM, Good News Translation, June 2003
\usfm 3.0
```
*/
pub struct USFM;
