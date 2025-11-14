/**
# `\lit`

[Source](https://ubsicap.github.io/usfm/characters/index.html#lit)

- **Syntax**: `\lit_text...`
- **Type**: `paragraph`
- **Added**: `1.0`
- **Use**: Liturgical note/comment. (e.g. a guide which tells the reader/worshipper that he should recite a prayer or recitation etc.) \
    *A paragraph style.*

**Text and Formatting Sample** - Psalm 3 (Russian Synodal, Orthodox Version)

```usfm
\c 3
\d
\v 1 Псалом Давида, когда он бежал от Авессалома, сына своего.
\p
\v 2 Господи! как умножились враги мои! Многие восстают на меня;
\v 3 многие говорят душе моей: «нет ему спасения в Боге».
\v 4 Но Ты, Господи, щит предо мною, слава моя, и Ты возносишь голову мою.
\v 5 Гласом моим взываю к Господу, и Он слышит меня со святой горы Своей.
\v 6 Ложусь я, сплю и встаю, ибо Господь защищает меня.
\v 7 Не убоюсь тем народа, которые со всех сторон ополчились на меня.
\v 8 Восстань, Господи! спаси меня, Боже мой! ибо Ты поражаешь в ланиту всех врагов
моих; сокрушаешь зубы нечестивых.
\v 9 От Господа спасение. Над народом Твоим благословение Твое.
\lit Слава:
```

*Слава: = "Glory".*
*/
#[derive(crate::Marker!)]
pub struct LIT;
