pub mod eval;
pub mod parse;
pub mod terms;
pub mod values;

use common::language::{untyped::Untyped, Language};
use terms::Term;
use values::Value;

pub struct UntypedLambda;

impl Language for UntypedLambda {
    type Term = Term;
    type Type = Untyped;
    type Value = Value;
    type CheckEnv = ();
    type EvalEnv = ();
}
