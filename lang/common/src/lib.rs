pub mod check;
pub mod errors;
pub mod eval;
pub mod langs;
pub mod parse;
pub mod subst;
pub use check::Typecheck;
pub use eval::Eval;
pub use parse::Parse;

pub type Var = String;
pub type TypeVar = String;
pub type Label = String;
pub type Kind = String;
pub type Term = String;
pub type Location = usize;
pub type Type = String;
pub type Value = String;
