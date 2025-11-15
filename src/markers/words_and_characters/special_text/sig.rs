/**
# `\sig ...\sig*`

[Source](https://ubsicap.github.io/usfm/characters/index.html#sig-sig)

- **Syntax**: `\sig_text...\sig*`
- **Type**: `character`
- **Added**: `1.0`
- **Use**: Signature of the author (of a letter or epistle).

**Text and Formatting Sample** - Colossians 4.18 (GNT)

```usfm
\p
\v 18 With my own hand I write this: \sig Greetings from Paul\sig*. Do not forget
my chains!
\cls May God's grace be with you.
```
*/
#[derive(crate::Marker!, crate::Deref!)]
pub struct SIG(bool);
crate::impl_paired_tag!(SIG, "sig");
