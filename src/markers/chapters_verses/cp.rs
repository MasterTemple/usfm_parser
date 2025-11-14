/**
# `\cp`

[Source](https://ubsicap.github.io/usfm/chapters_verses/index.html#cp)

- **Syntax**: `\cp_text...`
- **Type**: `paragraph`
- **Added**: `1.0`
- **Use**: Published chapter character. \
    This is the chapter character (number, letter) which should be displayed in the published text (where the published marker is different than the `\\c #` used within the translation editing environment).

**Text and Formatting Sample** - Esther-Greek 1 ("A") (GNT)

```usfm
\c 1
\cp A
\s1 Mordecai's Strange Dream
\p
\v 1-3 \va 2-4\va* Mordecai, a Jew who belonged to the tribe of Benjamin, was taken into
exile, along with King Jehoiachin of Judah, when King Nebuchadnezzar of Babylonia captured
Jerusalem. ...
```
*/
#[derive(crate::Marker!)]
pub struct CP;
