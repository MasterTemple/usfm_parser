/**
# `\xop ...\xop*`

[Source](https://ubsicap.github.io/usfm/notes_basic/xrefs.html#xop-xop)

- **Syntax**: `\xop_text...\xop*`
- **Type**: `character (note)`
- **Added**: `3.0`
- **Use**: Published cross reference origin text. \
    In some texts, the content intended to be published in the position of the cross reference origin text `\xo` does not follow the typical `` pattern. An origin reference following this pattern is required for validation of the cross reference location. `\xop ...\xop*` can be used in order to supply the content intended for publishing, similar to the use of `\cp` and `\vp ...\vp*`.

**Text and Formatting Sample** - Jonah 1.1-5 (Bulgarian Orthodox Bible)

```usfm
\p
\v 1 \x - \xo 1:1 \xop Гл 1. (1)\xop* \xt 4 Царств. 14:25.\x*И биде слово Господне
към Иона, син Аматиев:
\v 2 \x - \xo 1:2 \xop (2)\xop* \xt Бит. 10:11. Иона 3:3.\x*„стани, иди в Ниневия,
град голям, и проповядвай в него, защото злодеянията му достигнаха до Мене“.
\v 3 И стана Иона да побегне в Тарсис от лицето Господне; дойде в Иопия и намери кораб,
който отиваше за Тарсис, плати за превоз и влезе в него, за да отплува с тях в Тарсис
от лицето Господне.
\v 4 \x - \xo 1:4 \xop (4)\xop* \xt Пс. 106:25.\x*Но Господ подигна в морето силен
вятър, и стана в морето голяма буря, и корабът насмалко оставаше да се разбие.
\v 5 \x - \xo 1:5 \xop (5)\xop* \xt 4 Царств. 17:29.\x*Уплашиха се корабниците; те
викаха всеки към своя бог и почнаха да хвърлят в морето товара от кораба, за да му
олекне от него; а Иона бе слязъл в дъното на кораба, бе легнал и дълбоко заспал.
```
*/
#[derive(crate::Marker!)]
pub struct XOP;
crate::impl_paired_tag!(XOP, "xop");
