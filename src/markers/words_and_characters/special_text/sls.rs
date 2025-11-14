/**
# `\sls ...\\sls\*`

[Source](https://ubsicap.github.io/usfm/characters/index.html#sls-sls)

- **Syntax**: `\sls_text...\sls*`
- **Type**: `character`
- **Added**: `1.0`
- **Use**: Passage of text based on a secondary language or alternate text source. \
    E.g. The French NBS02 has large sections of text in EZR and DAN in italics, to represent where the original text is in Aramaic, not Hebrew.

**Text and Formatting Sample** - Ezra 4.8—6.18 (NBS - French, Nouvelle Bible Segond)

```usfm
\v 7 Et aux jours d'Artaxerxès, Bishlam, Mitredath, Tabéel et le reste de leurs
collègues écrivirent à Artaxerxès, roi de Perse. Le texte de la lettre fut écrit en
araméen, traduit en araméen.
\p
\v 8 \sls Rehoum, chancelier, et Shimshaï, secrétaire, écrivirent au roi Artaxerxès
la lettre suivante concernant Jérusalem, savoir:\sls*
\v 9 \sls «Rehoum, chancelier, Shimshaï, secrétaire, et le reste de leurs collègues,
ceux de Dîn, d'Apharsatak, de Tarpel, d'Apharas, d'Erek, de Babylone, de Suse, de Déha,
d'Elam,\sls*
...
```
*/
pub struct SLS;
