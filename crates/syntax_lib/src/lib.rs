pub mod errors;
pub mod kinds;
pub mod subst;
pub mod terms;
pub mod types;
pub mod untyped;

pub type Var = String;
pub type TypeVar = String;
pub type Label = String;
pub type Location = usize;
