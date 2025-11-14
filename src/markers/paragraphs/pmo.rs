/**
# `\pmo`

[Source](https://ubsicap.github.io/usfm/paragraphs/index.html#pmo)

- **Syntax**: `\pmo(_text...)`
- **Type**: `paragraph`
- **Added**: `2.0`
- **Use**: Embedded text opening. \

**Text and Formatting Sample** - Acts 15.24 (CEV)

```usfm
\p
\v 22 The apostles, the leaders, and all the church members decided to send some men to
Antioch along with Paul and Barnabas. They chose Silas and Judas Barsabbas, who were two
leaders of the Lord's followers.
\v 23 They wrote a letter that said:
\pmo We apostles and leaders send friendly greetings to all of you Gentiles who are
followers of the Lord in Antioch, Syria, and Cilicia.
\pm
\v 24 We have heard that some people from here have terribly upset you by what they said.
But we did not send them!
```
*/
#[derive(crate::Marker!)]
pub struct PMO;
