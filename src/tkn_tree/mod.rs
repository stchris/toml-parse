pub(self) use super::common::{
    err::{self, ParseTomlError, TomlErrorKind, TomlResult},
    munch::{self, Muncher, ARRAY_ITEMS, BOOL_END, DATE_LIKE, EOL, KEY_END, NUM_END},
};

mod kinds;
mod tokenize;
mod row;

pub use kinds::{TomlKind, TomlNode, TomlToken};
pub use tokenize::Tokenizer;

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;
    #[test]
    fn token_file() {
        // ftop.toml is 7 items long
        let input = read_to_string("examp/ftop.toml").expect("file read failed");
        let parsed = Tokenizer::parse(&input).expect("parse failed");
        // assert_eq!(parsed.len(), 7);
        println!("{:#?}", parsed);
    }

    #[test]
    fn parents() {
        let file = "[table]\n# hello there";
        let parsed = Tokenizer::parse(file).expect("parse failed");

        println!("{:#?}", parsed.as_node().unwrap());

        // for ele in parsed.walk_with_tokens() {
        //     println!("{:?}", ele);
        //     println!("{:?}", ele.ancestors().collect::<Vec<_>>())
        // }
    }

    #[test]
    fn all_tokens() {
        let file = r#"[deps]
alpha = "beta"
number = 1234
array = [ true, false, true ]
inline-table = { date = 1988-02-03T10:32:10, }
"#;
        let parsed = Tokenizer::parse(file).expect("parse failed");

        println!("{:#?}", parsed.as_node().unwrap());

        // for ele in parsed.walk_with_tokens() {
        //     println!("{:?}", ele);
        //     println!("{:?}", ele.ancestors().collect::<Vec<_>>())
        // }
    }
}