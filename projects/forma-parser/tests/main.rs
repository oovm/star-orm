use forma_core::ast::IdentifierNode;
use forma_parser::NoteParser;
use pex::ParseState;

#[test]
fn ready() {
    println!("it, works!")
}

#[test]
fn test() {
    let id = IdentifierNode::parse(ParseState::new("a_b"));
    // println!("{}", id);
    println!("{:#?}", id);
}
