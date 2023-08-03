use super::*;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SequenceNode {
    pub terms: Vec<SequenceTermNode>,
    pub span: Range<u32>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SequenceTermNode {
    Identifier(IdentifierNode),
    Ligature(LigatureNode),
    NumberLiteral(NumberLiteralNode),
    NumberValue(NumberValueNode),
}
