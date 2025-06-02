pub mod check;
pub mod errors;
pub mod eval;
pub mod parser;
pub mod terms;
pub mod types;
pub mod values;

use crate::Language;
use check::ExceptionEnv;
use errors::Error;
use terms::Term;
use types::Type;
use values::Value;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Exceptions;

impl Language for Exceptions {
    type Term = Term;
    type Type = Type;
    type Value = Value;
    type CheckEnv = ExceptionEnv;
    type EvalEnv = ();
    type LanguageError = Error;
}
