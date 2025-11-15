/**
# `\vp ...\vp*`

[Source](https://ubsicap.github.io/usfm/chapters_verses/index.html#vp-vp)

- **Syntax**: `\vp text...\vp*`
- **Type**: `character`
- **Added**: `1.0`
- **Use**: Published verse character. \
    This is the verse character (number, letter) which should be displayed in the published text (where the published character(s) are different than the `\v #` digit used within the translation editing environment).

**Text and Formatting Sample** - Esther-Greek 3.14,15 ("Addition B") (CEV)

```usfm
\cp 13
\ms1 Addition B
\s A Copy of the Letter
\p
\v 14 \vp 1b\vp* This is a copy of the letter:
\pmo From Artaxerxes, the Great King, to the governors and officials of my one hundred
twenty-seven provinces from India to Ethiopia.
\pm
\v 15 \vp 2b\vp* I rule many nations, and I am the most powerful king in the world.
But I have never used my power in a proud or arrogant way. Instead, I have always
been reasonable and kind to the people in my kingdom. I know they want peace, and so
I have decided to make every part of my kingdom peaceful and safe for travel.
```

*/
#[derive(crate::Marker!, crate::Deref!)]
pub struct VP(bool);
crate::impl_paired_tag!(VP, "vp");
