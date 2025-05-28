pub mod check;
pub mod eval;
pub mod parser;
pub mod terms;
pub mod types;
pub mod values;

use crate::Language;
use check::Env;
use terms::Term;
use types::Type;
use values::Value;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LambdaOmega;

impl Language for LambdaOmega {
    type Term = Term;
    type Type = Type;
    type Value = Value;
    type CheckEnv = Env;
    type EvalEnv = ();
}
