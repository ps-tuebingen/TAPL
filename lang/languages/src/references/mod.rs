pub mod check;
pub mod eval;
pub mod parser;
pub mod terms;
pub mod types;
pub mod values;

use check::Environment;
use common::language::Language;
use eval::Store;
use terms::Term;
use types::Type;
use values::Value;

pub struct References;

impl Language for References {
    type Term = Term;
    type Type = Type;
    type Value = Value;
    type CheckEnv = Environment;
    type EvalEnv = Store;
}
