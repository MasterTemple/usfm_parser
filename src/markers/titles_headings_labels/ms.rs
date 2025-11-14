/**
# `\ms#`

[Source](https://ubsicap.github.io/usfm/titles_headings/index.html#ms)

- **Syntax**: `\ms#_text...`
- **Type**: `paragraph`
- **Added**: `1.0`
- **Use**: Major section heading. \
    These are headings before larger text divisions than what is typically considered a "section" division (see `\\s#`). \
    The variable # represents the level of division. \
    **\\ms = \\ms1** (see `syntax notes` on numbered markers)

**Text and Formatting Samples** - Psalm 1 (Book 1 division) (GNT)

```usfm
\c 1
\ms BOOK ONE
\mr (Psalms 1–41)
\s True Happiness
\q1
\v 1 Happy are those
\q2 who reject the advice of evil people,
```

Daniel 1.1 (GNT)

```usfm
\c 1
\ms THE STORY OF DANIEL AND HIS FRIENDS
\mr (1.1—6.28)
\s The Young Men at Nebuchadnezzar's Court
\p
\v 1 In the third year that Jehoiakim was king of Judah, King Nebuchadnezzar of Babylonia
attacked Jerusalem and surrounded the city.
```
*/
#[derive(crate::Marker!)]
pub struct MS;
