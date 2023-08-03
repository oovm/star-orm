mod identifier;
// mod ignore;

mod command;
mod number;

use crate::{helpers::get_span, traits::ThisParser};
use forma_core::ast::{IdentifierNode, LigatureNode};
use pex::{ParseResult, ParseState, Regex};
use std::sync::LazyLock;
