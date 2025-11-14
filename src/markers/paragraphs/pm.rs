/**
# `\pm`

[Source](https://ubsicap.github.io/usfm/paragraphs/index.html#pm)

- **Syntax**: `\pm(_text...)`
- **Type**: `paragraph`
- **Added**: `2.0`
- **Use**: Embedded text paragraph.

**Text and Formatting Sample** - Act 15.24-27,28-29 (CEV)

```usfm
\pmo We apostles and leaders send friendly greetings to all of you Gentiles who are
followers of the Lord in Antioch, Syria, and Cilicia.
\pm
\v 24 We have heard that some people from here have terribly upset you by what they said.
But we did not send them!
\v 25 So we met together and decided to choose some men and to send them to you along with
our good friends Barnabas and Paul.
\v 26 These men have risked their lives for our Lord Jesus Christ.
\v 27 We are also sending Judas and Silas, who will tell you in person the same things that
we are writing.
\pm
\v 28 The Holy Spirit has shown us that we should not place any extra burden on you...
```
*/
#[derive(crate::Marker!)]
pub struct PM;
