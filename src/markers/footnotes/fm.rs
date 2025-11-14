/**
# `\fm ...\\fm\*`

[Source](https://ubsicap.github.io/usfm/notes_basic/fnotes.html#fm-fm)

- **Syntax**: `\fm_text...\fm*`
- **Type**: `character (note)`
- **Added**: `1.0`
- **Use**: Footnote reference mark. \
    Used where two or more locations in the scripture text should ideally refer the reader to the same footnote text (as seen in identical footnote text which is referenced at Gen 2.9 and Gen 2.17 in some translations).

## Warning

> Because the nature of this marker is related directly to the published form of the text, it is not intended for use in scripture authoring. It may be used during the publishing process to connect two callers to the same footnote text.
*/
pub struct FM;
