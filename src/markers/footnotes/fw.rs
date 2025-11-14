/**
# `\fw`

[Source](https://ubsicap.github.io/usfm/notes_basic/fnotes.html#fw)

- **Syntax**: `\fw_text...`
- **Type**: `character (note)`
- **Added**: `3.0`
- **Use**: Footnote witness list. \
    For distinguishing a list of sigla representing witnesses in critical editions.

## Note

> Apparatus entries of printed critical editions are densely packed with information. One key part is the list of witnesses supporting a specific reading. The witnesses are usually represented by sigla consisting of one character, an abbreviation, or a number. It can be very helpful to distinguish witness lists from other footnote text, which can make it simpler to introduce checking tools for these lists, and to create linking and reader helps in digital representations.

**Text and Formatting Samples** - Matthew 28.14 (Nestle-Aland 29)

```usfm
\f ⸀ \fr 28,14 \ft υπο \fw B D 0148. 892\f*
```

Matthew 4.1 (Nestle-Aland 29)

```usfm
\f ° \fr 4,1 \fw B Δ 700\f*
```
*/
#[derive(crate::Marker!)]
pub struct FW;
crate::impl_simple_tag!(FW, "fw");
