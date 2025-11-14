/**
# `\fig ...\\fig\*`

[Source](https://ubsicap.github.io/usfm/characters/index.html#fig-fig)

- **Syntax**: `\fig_caption text...|src="filename" size="size" ref="reference"\fig*`
- **Type**: `paragraph`
- **Added**: `1.0`
- **Updated**: 3.0 (attributes)
- **Use**: For defining illustrations to be used within a publication.

.. caution:: |badge_3.0| **Significant syntax change from USFM 1.x / 2.x**

    The syntax for defining illustrations in USFM 3.0 follows the general syntax for providing `word level attributes`. In USFM 1.x and 2.x, markup for illustrations required a the content for a collection of parameters to be provided in a specific order, with items separated by a vertical bar (e.g `\fig_DESC|FILE|SIZE|LOC|COPY|CAP|REF\fig*`). The use of marker attributes, and the use of a vertical bar as an attribute separator was unique to illustration markup in USFM 1.x and 2.x. In USFM 3.0 this syntax is deprecated in order to align the markup with the general syntax for `word level attributes`.
*/
#[derive(crate::Marker!)]
pub struct FIG;
