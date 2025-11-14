/**
# `\add ...\add*`

[Source](https://ubsicap.github.io/usfm/characters/index.html#add-add)

- **Syntax**: `\add_text...\add*`
- **Type**: `character`
- **Added**: `1.0`
- **Use**: Translator's addition. \
    A translator's explanation; words added by the translator for clarity – text which is not literally a part of the original language, but which was supplied to make the meaning of the original clear.

**Text and Formatting Sample** - Genesis 5.29 (Russian Synodal, Protestant Version)

```usfm
\p
\v 29 И нарек ему имя: Ной, сказав: он утешит нас в работе нашей и в трудах рук
наших при \add возделывании\add* земли, которую проклял Господь.
```
*/
#[derive(crate::Marker!)]
pub struct ADD;
