mod identifier;
// mod ignore;

mod command;

use std::sync::LazyLock;
use pex::{ParseResult, ParseState, Regex};
use crate::traits::ThisParser;
use forma_core::ast::IdentifierNode;
