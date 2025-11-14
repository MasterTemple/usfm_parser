/**
# `\c`

[Source](https://ubsicap.github.io/usfm/chapters_verses/index.html#c)

- **Syntax**: `\c_#`
- **Type**: `paragraph`
- **Added**: `1.0`
- **Use**: Chapter number. \
    The marker is followed by the chapter number #. \
    No further text should follow this marker.

**Text and Formatting Sample** - Matthew 1 (GNT)

```usfm
\io1 The last week in and near Jerusalem (21.1–27.66)
\io1 The resurrection and appearances of the Lord (28.1-20)
\c 1
\s1 The Ancestors of Jesus Christ
\r (Luke 3.23-38)
\p
\v 1 This is the list of the ancestors of Jesus Christ, a descendant of David, who was a
descendant of Abraham.
```
*/
#[derive(crate::Marker!)]
pub struct C(usize);

#[test]
fn c() {
    let a = C(1);
    let b = a.clone();
    assert_eq!(a, b);
}
