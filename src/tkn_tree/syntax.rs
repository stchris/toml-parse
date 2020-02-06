use std::fmt;

use rowan::{GreenNode, GreenNodeBuilder, SmolStr};

use super::err::{ParseTomlError, TomlErrorKind, TomlResult};
use super::kinds::TomlKind::{self, *};
use super::parse_tkns::Tokenizer;
use super::walk;

pub type SyntaxNode = rowan::SyntaxNode<TomlLang>;
pub type SyntaxToken = rowan::SyntaxToken<TomlLang>;
pub type SyntaxElement = rowan::NodeOrToken<SyntaxNode, SyntaxToken>;

pub trait Printer {
    fn print_text(&self) -> String;
}
impl From<TomlKind> for rowan::SyntaxKind {
    fn from(kind: TomlKind) -> Self {
        Self(kind as u16)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TomlLang;
impl rowan::Language for TomlLang {
    type Kind = TomlKind;
    fn kind_from_raw(raw: rowan::SyntaxKind) -> Self::Kind {
        assert!(raw.0 <= Root as u16);
        unsafe { std::mem::transmute::<u16, TomlKind>(raw.0) }
    }
    fn kind_to_raw(kind: Self::Kind) -> rowan::SyntaxKind {
        kind.into()
    }
}

impl Printer for SyntaxNode {
    /// walks tokens collecting each tokens text into a final String.
    fn print_text(&self) -> String {
        walk::walk_tokens(self).fold(String::default(), |mut s, tkn| {
            s.push_str(tkn.text());
            s
        })
    }
}

pub struct ParsedToml {
    green: rowan::GreenNode,
}

impl ParsedToml {
    pub fn syntax(&self) -> SyntaxNode {
        SyntaxNode::new_root(self.green.clone())
    }
}

pub struct Parser {
    /// the in-progress tree.
    pub(crate) builder: GreenNodeBuilder<'static>,
}

impl Default for Parser {
    fn default() -> Self {
        Parser::new()
    }
}

impl Parser {
    pub fn new() -> Parser {
        Self {
            builder: GreenNodeBuilder::new(),
        }
    }
    pub fn parse(self) -> TomlResult<ParsedToml> {
        let green: GreenNode = self.builder.finish();
        // Construct a `SyntaxNode` from `GreenNode`,
        // using errors as the root data.
        Ok(ParsedToml { green })
    }
}

pub fn parse_it(input: &str) -> TomlResult<ParsedToml> {
    let parse_builder = Parser::new();
    let parsed = Tokenizer::parse(input, parse_builder)?;
    parsed.parse()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn parents() {
        let file = "[table]\n# hello there";
        let parsed = parse_it(file).expect("parse failed");
        let root = parsed.syntax();
        assert_eq!(root.print_text(), file)
    }

    #[test]
    fn single_quote_key() {
        let file = "[table]\n'key' = \"value\"";
        let parsed = parse_it(file).expect("parse failed");
        let root = parsed.syntax();
        assert_eq!(root.print_text(), file)
    }

    #[test]
    fn double_quote_key() {
        let file = "[table]\n\"key\" = \"value\"";
        let parsed = parse_it(file).expect("parse failed");
        let root = parsed.syntax();
        assert_eq!(root.print_text(), file)
    }

    #[test]
    fn single_quote_value() {
        let file = "[table]\nkey = 'value'";
        let parsed = parse_it(file).expect("parse failed");
        let root = parsed.syntax();
        assert_eq!(root.print_text(), file)
    }

    #[test]
    fn triple_quote_value() {
        let file = "[table]\nkey = \"\"\"value\"\"\"";
        let parsed = parse_it(file).expect("parse failed");
        let root = parsed.syntax();
        assert_eq!(root.print_text(), file)
    }

    #[test]
    fn triple_quote_value_complex() {
        let file = "[table]\nkey = \"\"\"value \"hello\" bye\n end\"\"\"";
        let parsed = parse_it(file).expect("parse failed");
        let root = parsed.syntax();
        assert_eq!(root.print_text(), file)
    }

    #[test]
    fn all_tokens() {
        let file = r#"[deps]
alpha = "beta"
number = 1234
array = [ true, false, true ]
inline-table = { date = 1988-02-03T10:32:10, }
"#;
        let parsed = parse_it(file).expect("parse failed");
        assert_eq!(parsed.syntax().print_text(), file)
    }

    #[test]
    fn ftop_file() {
        // ftop.toml is 7 items long
        let input = read_to_string("examp/ftop.toml").expect("file read failed");
        let parsed = parse_it(&input).expect("parse failed");
        assert_eq!(parsed.syntax().print_text(), input)
    }
    #[test]
    fn fend_file() {
        // ftop.toml is 7 items long
        let input = read_to_string("examp/fend.toml").expect("file read failed");
        let parsed = parse_it(&input).expect("parse failed");
        assert_eq!(parsed.syntax().print_text(), input)
    }
    #[test]
    fn seg_file() {
        // ftop.toml is 7 items long
        let input = read_to_string("examp/seg.toml").expect("file read failed");
        let parsed = parse_it(&input).expect("parse failed");
        assert_eq!(parsed.syntax().print_text(), input)
    }
    #[test]
    fn work_file() {
        // ftop.toml is 7 items long
        let input = read_to_string("examp/work.toml").expect("file read failed");
        let parsed = parse_it(&input).expect("parse failed");
        assert_eq!(parsed.syntax().print_text(), input)
    }

    #[test]
    fn print_print_text() {
        // ftop.toml is 7 items long
        let input = read_to_string("examp/ftop.toml").expect("file read failed");
        let root = parse_it(&input).expect("parse failed").syntax();
        assert_eq!(root.print_text(), input)
    }
}