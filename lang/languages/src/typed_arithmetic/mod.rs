pub mod check;
pub mod eval;
pub mod parser;
pub mod terms;
pub mod types;
pub mod values;

use check::Env;
use common::language::Language;
use terms::Term;
use types::Type;
use values::Value;

pub struct TypedArithmetic;

impl Language for TypedArithmetic {
    type Term = Term;
    type Type = Type;
    type Value = Value;
    type CheckEnv = Env;
    type EvalEnv = ();
}
