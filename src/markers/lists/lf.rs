/**
# `\lf`

[Source](https://ubsicap.github.io/usfm/lists/index.html#lf)

- **Syntax**: `\lf_text...`
- **Type**: `paragraph`
- **Added**: `3.0`
- **Use**: List footer. \
    Some lists include an introductory (`\\lh`) and concluding remark. They are an integral part of the list content, but are not list items. A list does not require either or both of these elements.

**Text and Formatting Sample** - 1 Chronicles 27:16-22 (GNT - *markup adapted*)

```usfm
\s1 Administration of the Tribes of Israel
\lh
\v 16-22 This is the list of the administrators of the tribes of Israel:
\li1 Reuben - Eliezer son of Zichri
\li1 Simeon - Shephatiah son of Maacah
\li1 Levi - Hashabiah son of Kemuel
\li1 Aaron - Zadok
\li1 Judah - Elihu, one of King David's brothers
\li1 Issachar - Omri son of Michael
\li1 Zebulun - Ishmaiah son of Obadiah
\li1 Naphtali - Jeremoth son of Azriel
\li1 Ephraim - Hoshea son of Azaziah
\li1 West Manasseh - Joel son of Pedaiah
\li1 East Manasseh - Iddo son of Zechariah
\li1 Benjamin - Jaasiel son of Abner
\li1 Dan - Azarel son of Jeroham
\lf This was the list of the administrators of the tribes of Israel.
```
*/
pub struct LF;
