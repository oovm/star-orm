use super::*;

pub struct IdentifierNode {
    pub name: String,
    pub span: Range<u32>,
}