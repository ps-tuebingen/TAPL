pub mod check;
pub mod eval;
pub mod parser;
pub mod terms;
pub mod types;
pub mod values;

use common::{language::Language, Var};
use std::collections::HashMap;
use terms::Term;
use types::Type;
use values::Value;

pub struct Stlc;

impl Language for Stlc {
    type Term = Term;
    type Type = Type;
    type Value = Value;
    type CheckEnv = HashMap<Var, Type>;
    type EvalEnv = ();
}
