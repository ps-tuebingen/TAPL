pub mod definition;
pub mod kinds;
pub mod language;
pub mod program;
pub mod span;
pub mod terms;
pub mod types;
pub mod values;

pub mod env;
pub mod eval_context;
pub mod free_vars;
pub mod subst;
pub mod untyped;

pub type Name = String;
pub type Label = String;
pub type Var = String;
pub type Location = usize;
pub type TypeVar = String;
pub type KindVar = String;
