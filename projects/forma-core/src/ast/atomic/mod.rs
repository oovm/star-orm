use super::*;

#[derive(Debug)]
pub struct IdentifierNode {
    pub name: String,
    pub span: Range<u32>,
}

impl IdentifierNode {
    pub fn new<S: ToString>(body: S, span: Range<u32>) -> Self {
        Self { name: body.to_string(), span }
    }
}