use super::*;
use forma_core::ast::TextNode;

// no whitespace
// no newline
// no math $
// no comma , dot
#[rustfmt::skip]
pub static TEXT: LazyLock<Regex> = LazyLock::new(|| {Regex::new(r#"^(?x)(
    [^,.$\s\\\[]+
)"#).unwrap()});

impl ThisParser for TextNode {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, m) = input.match_regex(&TEXT, "IDENTIFIER")?;
        let number = TextNode::new(m.as_str(), get_span(input, state));
        state.finish(number)
    }
}

#[test]
fn test() {
    let id = TextNode::parse(ParseState::new("a_b, good"));
    // println!("{}", id);
    println!("{:#?}", id);
}
