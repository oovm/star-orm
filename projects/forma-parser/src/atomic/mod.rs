mod identifier;
// mod ignore;

mod command;
mod infix;
mod number;
mod text;

use crate::{helpers::get_span, traits::NoteParser};
use forma_core::ast::{IdentifierNode, LigatureNode};
use pex::{ParseResult, ParseState, Regex};
use std::sync::LazyLock;
