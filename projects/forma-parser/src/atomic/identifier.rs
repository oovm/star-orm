use forma_core::ast::LigatureNode;
use crate::helpers::get_span;
use super::*;

#[rustfmt::skip]
pub static IDENTIFIER: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^(?x)(
    \p{XID_START}[\p{XID_Continue}&&[^_＿]]+
)").unwrap()
});

pub static LIGATURE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^(?x)(
    != # ≠
|   >= # ⩾
|   <= # ⩽
|   \+- # ±
|   -\+ # ∓
|   -> # →
|   => # ⇒
|   ->> # ↠
|   >-> # ↣
|   \|-> # ↦
|   ~>> # ⇝
|   ~> # \leadsto
)").unwrap()
});

impl ThisParser for IdentifierNode {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, m) = input.match_regex(&IDENTIFIER, "IDENTIFIER")?;
        let id = IdentifierNode::new(m.as_str(), get_span(input, state));
        state.finish(id)
    }
}

impl ThisParser for LigatureNode {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, m) = input.match_regex(&IDENTIFIER, "IDENTIFIER")?;
        let id = IdentifierNode::new(m.as_str(), get_span(input, state));
        state.finish(id)
    }
}

#[test]
fn test() {
    let id = IdentifierNode::parse(ParseState::new("a_b"));
    // println!("{}", id);
    println!("{:#?}", id);
}
