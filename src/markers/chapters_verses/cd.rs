/**
# `\cd`

[Source](https://ubsicap.github.io/usfm/chapters_verses/index.html#cd)

- **Syntax**: `\cd_text...`
- **Type**: `paragraph`
- **Added**: `1.0`
- **Use**: Chapter description. \
    A brief description of chapter content (similar to `\\d` - descriptive heading, or `\\iex` - explanatory or bridge text).

**Text and Formatting Sample** - Genesis 2 (Russian Synodal, Protestant Version)

```usfm
\c 2
\cd 1 Бог благословляет седьмой день; 8 человек в раю Едемском; четыре реки; дерево
познания добра и зла. 18 Человек дает названия животным. 21 Создание женщины.
\p
\v 1 Так совершены небо и земля и все воинство их.
\p
\v 2 И совершил Бог к седьмому дню дела Свои, которые Он делал, и почил в день седьмой
от всех дел Своих, которые делал.
```
*/
pub struct CD;
