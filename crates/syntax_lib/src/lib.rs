pub mod kinds;
pub mod subst;
pub mod terms;
pub mod types;
pub mod untyped;

pub use common::Label;
pub use common::Var;

pub type TypeVar = String;
pub type Location = usize;
