pub mod definition;
pub mod env;
pub mod eval_context;
pub mod kinds;
pub mod language;
pub mod program;
pub mod subst;
pub mod terms;
pub mod types;
pub mod untyped;
pub mod values;

pub type Name = String;
pub type Label = String;
pub type Var = String;
pub type Location = usize;
pub type TypeVar = String;
