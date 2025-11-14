/**
# `\thr#`

[Source](https://ubsicap.github.io/usfm/tables/index.html#thr)

- **Syntax**: `\thr#_text...`
- **Type**: `character`
- **Added**: `1.0 (updated 3.0 - column spanning)`
- **Use**: Right aligned table column heading. \
    The variable # represents the table column number. \ \
    |badge_3.0| Use a dash `-` between a range of column numbers to indicate that the columns should be spanned.

**Text and Formatting Samples** - Numbers 7.12-83 (GNT)

```usfm
\p
\v 12-83 They presented their offerings in the following order:
\tr \th1 Day \th2 Tribe \th3 Leader
\tr \tcr1 1st \tc2 Judah \tc3 Nahshon son of Amminadab
\tr \tcr1 2nd \tc2 Issachar \tc3 Nethanel son of Zuar
\tr \tcr1 3rd \tc2 Zebulun \tc3 Eliab son of Helon
\tr \tcr1 4th \tc2 Reuben \tc3 Elizur son of Shedeur
\tr \tcr1 5th \tc2 Simeon \tc3 Shelumiel son of Zurishaddai
...
```

Numbers 2.10-16 (GNT)

```usfm
\p
\v 10-16 On the south, those under the banner of the division of Reuben shall camp in
their groups, under their leaders, as follows:
\tr \th1 Tribe \th2 Leader \thr3 Number
\tr \tc1 Reuben \tc2 Elizur son of Shedeur \tcr3 46,500
\tr \tc1 Simeon \tc2 Shelumiel son of Zurishaddai \tcr3 59,300
\tr \tc1 Gad \tc2 Eliasaph son of Deuel \tcr3 45,650
\tr \tcr1-2 Total: \tcr3 151,450
```
*/
#[derive(crate::Marker!)]
pub struct THR;
