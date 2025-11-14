/**
# `\x ...\x*`

[Source](https://ubsicap.github.io/usfm/notes_basic/xrefs.html#x-x)

- **Syntax**: `\x_+_(\xo_REF_)cross reference content\x*`
- **Type**: `note`
- **Added**: `1.0`
- **Use**: Beginning and ending of the cross reference element.

* The cross reference caller, which may be one of the following three types:
    - `+` – indicates that the caller should be generated automatically by the translation editor, or publishing tools. \
    - `-` – indicates that no caller should be generated, and is not used. \
    - `?` – where ? represents the character to be used for the caller. The caller is defined for the specific cross reference by the author.

- `cross reference content` (see `below`)
    - All of the text elements which make up the cross reference:
        - `origin` reference
        - special cross reference elements such as keywords or quotations
        - `target` references
    - Each element should be prefixed by the appropriate marker (listed below).

## Note

> **Important:** See `Syntax Notes` for addition information on the use of `endmarkers` for elements within cross reference content.
*/
#[derive(crate::Marker!)]
pub struct X;
crate::impl_paired_tag!(X, "x");
