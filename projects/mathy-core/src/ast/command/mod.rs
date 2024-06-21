use super::*;

/// Command `\a(b, c):`
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CommandLineNode {
    pub name: IdentifierNode,
    pub rest: TextNode,
    pub span: Range<u32>,
}

/// Command `\a(b, c) { }`
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CommandBlockNode {
    pub name: IdentifierNode,
    pub span: Range<u32>,
}
