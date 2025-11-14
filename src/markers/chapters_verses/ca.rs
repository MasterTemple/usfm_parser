/**
# `\ca ...\ca\*`

[Source](https://ubsicap.github.io/usfm/chapters_verses/index.html#ca-ca)

- **Syntax**: `\ca_#\ca*`
- **Type**: `character`
- **Added**: `1.0`
- **Use**: Alternate chapter number. \
    Used for marking the chapter number used in an alternate versification scheme. Required when different versification traditions need to be supported in the same translation text. \
    The content within the marker pair should only contain the alternate chapter number, and not include any formatting/

**Text and Formatting Sample** - Psalm 54 (GNT - *markup adapted*)

```usfm
\c 54
\ca 53\ca*
\s1 A Prayer for Protection from Enemies
\d \va 1\va* A poem by David, \va 2\va* after the men from Ziph went to Saul and told him
that David was hiding in their territory.
\q1
\v 1 \va 3\va* Save me by your power, O God;
\q2 set me free by your might!
\q1
\v 2 \va 4\va* Hear my prayer, O God;
\q2 listen to my words!
```

*/
#[derive(crate::Marker!)]
pub struct CA;
