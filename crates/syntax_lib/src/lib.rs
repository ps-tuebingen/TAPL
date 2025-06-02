pub mod kinds;
pub mod subst;
pub mod terms;
pub mod types;
pub mod untyped;

pub use common::{Label, Location, Var};

pub type TypeVar = String;
