pub mod check;
pub mod eval;
pub mod parser;
pub mod terms;
pub mod types;
pub mod values;

use check::TypingContext;
use common::language::Language;
use terms::Term;
use types::Type;
use values::Value;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Subtypes;

impl Language for Subtypes {
    type Term = Term;
    type Type = Type;
    type Value = Value;
    type CheckEnv = TypingContext;
    type EvalEnv = ();
}
