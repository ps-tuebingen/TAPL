pub mod check;
pub mod errors;
pub mod eval;
pub mod parser;
pub mod terms;
pub mod types;
pub mod values;

use crate::Language;
use errors::Error;
use std::collections::HashMap;
use syntax::Var;
use terms::Term;
use types::Type;
use values::Value;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Recursive;

impl Language for Recursive {
    type Term = Term;
    type Type = Type;
    type Value = Value;
    type CheckEnv = HashMap<Var, Type>;
    type EvalEnv = ();
    type LanguageError = Error;
}
