#![feature(lazy_cell)]

mod atomic;
mod helpers;
pub mod notedown;
mod traits;

pub use self::traits::NoteParser;
