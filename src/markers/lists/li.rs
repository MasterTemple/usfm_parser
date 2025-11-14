/**
# `\li#`

[Source](https://ubsicap.github.io/usfm/lists/index.html#li)

- **Syntax**: `\li#(_text...)`
- **Type**: `paragraph`
- **Added**: `1.0`
- **Use**: List entry. \
    An out-dented paragraph meant to highlight the items of a list. \
    Lists may be used to markup the individual entries of a structure such as the days within the creation account, or the Decalogue (10 commandments). \
    The variable # represents the level of indent. \
    **\li = \li1** (see `syntax notes` on numbered markers)

**Text and Formatting Sample** - Numbers 7.84-88 (GNT)

```usfm
\p
\v 84-88 The totals of the offerings brought by the twelve leaders for the dedication of
the altar were as follows:
\li –twelve silver bowls and twelve silver basins weighing a total of 60 pounds
\li –twelve gold dishes weighing a total of 48 ounces, filled with incense
\li –twelve bulls, twelve rams, and twelve one-year-old lambs, plus the grain offerings that
go with them, for the burnt offerings
\li –twelve goats for the sin offerings
\li –twenty-four bulls, sixty rams, sixty goats, sixty one-year-old lambs, for the fellowship
offerings
```
*/
#[derive(crate::Marker!)]
pub struct LI;
