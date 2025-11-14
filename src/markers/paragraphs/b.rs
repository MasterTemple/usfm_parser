/**
# `\b`

[Source](https://ubsicap.github.io/usfm/paragraphs/index.html#b)

- **Syntax**: `\b`
- **Type**: `paragraph`
- **Added**: `1.0`
- **Use**: Blank line. \
    May be used to explicitly indicate additional white space between paragraphs. \
    *See also:* Poetry elements `\b` (used for stanza breaks in poetry, or between poetry and prose).

## Warning

> No text should follow this marker, and it should not be used before or after titles to indicate white-space.
*/
#[derive(crate::Marker!)]
pub struct B;
crate::impl_simple_tag!(B, "b");
