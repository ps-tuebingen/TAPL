pub mod errors;
pub mod eval;
pub mod parse;
pub mod terms;
pub mod values;

use crate::Language;
use errors::Error;
use syntax::untyped::Untyped;
use terms::Term;
use values::Value;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UntypedLambda;

impl Language for UntypedLambda {
    type Term = Term;
    type Type = Untyped;
    type Value = Value;
    type EvalEnv = ();
    type LanguageError = Error;
}
