use super::*;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct IdentifierNode {
    pub name: String,
    pub span: Range<u32>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LigatureNode {
    pub name: String,
    pub span: Range<u32>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NumberNode {
    pub name: String,
    pub span: Range<u32>,
}

impl IdentifierNode {
    pub fn new<S: ToString>(body: S, span: Range<u32>) -> Self {
        Self { name: body.to_string(), span }
    }
}

impl LigatureNode {
    pub fn new<S: ToString>(body: S, span: Range<u32>) -> Self {
        Self { name: body.to_string(), span }
    }
}

impl NumberNode {
    pub fn new<S: ToString>(body: S, span: Range<u32>) -> Self {
        Self { name: body.to_string(), span }
    }
}
