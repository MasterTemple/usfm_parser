/*!
# Structured List Entries

[Source](https://ubsicap.github.io/usfm/lists/index.html#lik-lik)

Standard USFM `table` structures can be challenging to display on small page sizes, or digital device displays. Scripture content is sometimes encoded within a USFM table in order to suggest a meaningful presentation, but the encoded presentation may only be rendered accurately or legibly in a larger format. The following character marker pairs can be used to create structured list entries which identify a set of related content, but do not encode a specific presentation.

## Note

> Structured lists are not strictly a replacement for table markup, but may prove to be a more flexible option for some types of tabular content.

Character marker pairs `\lik ...\lik\*` and `\liv# ...\liv#\*` mark the content of list entries (`\li`) which are essentially a key + value pair. A key may have multiple values.
*/

pub mod lik;
pub mod liv;
