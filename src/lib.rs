#[macro_use]
extern crate macro_rules_attribute;
pub mod derive_aliases;
pub(crate) use derive_aliases::*;

pub mod categories;
pub mod document;
pub mod ids;
pub mod markers;
pub mod parser;

#[cfg(test)]
mod tests {
    use super::*;

    /**
    Parse:
    ```
    \id hab 45HABGNT92.usfm, Good News Translation, June 2003
    \c 3
    \s1 A Prayer of Habakkuk
    \p
    \v 1 This is a prayer of the prophet Habakkuk:
    \b
    \q1
    \v 2 O \nd Lord\nd*, I have heard of what you have done,
    \q2 and I am filled with awe.
    \q1 Now do again in our times
    \q2 the great deeds you used to do.
    \q1 Be merciful, even when you are angry.
    ```

    Into:
    ```json
    {   "type": "USJ",
        "version": "3.1",
        "content": [
            {   "type": "book",
                "marker": "id",
                "content": ["45HABGNT92.usfm, Good News Translation, June 2003"],
                "code": "HAB" },
            {   "type": "chapter",
                "marker": "c",
                "number": "3",
                "sid": "HAB 3" },
            {   "type": "para",
                "marker": "s1",
                "content": ["A Prayer of Habakkuk\n"] },
            {   "type": "para",
                "marker": "p",
                "content": [
                    {   "type": "verse",
                        "marker": "v",
                        "number": "1",
                        "sid": "HAB 3:1" },
                    "This is a prayer of the prophet Habakkuk:\n"
                ] },
            {   "type": "para",
                "marker": "b" },
            {   "type": "para",
                "marker": "q1",
                "content": [
                    {   "type": "verse",
                        "marker": "v",
                        "number": "2",
                        "sid": "HAB 3:2" },
                    "O ",
                    {   "type": "char",
                        "marker": "nd",
                        "content": [ "Lord" ] },
                    ", I have heard of what you have done,\n"
                ] },
            {   "type": "para",
                "marker": "q2",
                "content": [ "and I am filled with awe.\n" ] },
            {   "type": "para",
                "marker": "q1",
                "content": [ "Now do again in our times\n" ] },
            {   "type": "para",
                "marker": "q2",
                "content": [ "the great deeds you used to do.\n" ] },
            {   "type": "para",
                "marker": "q1",
                "content": [ "Be merciful, even when you are angry." ] }
        ]
    }
    ``

    */
    #[test]
    fn readme1() {
        let usfm = r##"\id hab 45HABGNT92.usfm, Good News Translation, June 2003
\c 3
\s1 A Prayer of Habakkuk
\p
\v 1 This is a prayer of the prophet Habakkuk:
\b
\q1
\v 2 O \nd Lord\nd*, I have heard of what you have done,
\q2 and I am filled with awe.
\q1 Now do again in our times
\q2 the great deeds you used to do.
\q1 Be merciful, even when you are angry.
"##;
        // f
    }
}
