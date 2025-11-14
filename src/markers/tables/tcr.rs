/**
# `\tcr#`

[Source](https://ubsicap.github.io/usfm/tables/index.html#trc)

- **Syntax**: `\tcr#_text...`
- **Type**: `character`
- **Added**: `1.0 (updated 3.0 - column spanning)`
- **Use**: Right aligned table cell. \
    The variable # represents the table column number. \ \
    |badge_3.0| Use a dash `-` between a range of column numbers to indicate that the columns should be spanned.

**Text and Formatting Sample** - Numbers 2.10-16 (GNT)

```usfm
\p
\v 10-16 On the east side, those under the banner of the division of Judah shall camp in
their groups, under their leaders, as follows:
\tr \th1 Tribe \th2 Leader \thr3 Number
\tr \tc1 Judah \tc2 Nahshon son of Amminadab \tcr3 74,600
\tr \tc1 Issachar \tc2 Nethanel son of Zuar \tcr3 54,400
\tr \tc1 Zebulun \tc2 Eliab son of Helon \tcr3 57,400
\tr \tcr1-2 Total: \tcr3 186,400
```

## Warning

> **Empty table cells:**
> An empty table cell still requires a corresponding marker in the table text. Alternatively, indicate that a cell spans multiple columns by indicating a column range `\tr \tcr1-2 Total: \tcr3 151,450`.
*/
pub struct TCR;
