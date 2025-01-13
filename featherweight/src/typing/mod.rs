use crate::syntax::{ClassName, Var};
use std::collections::HashMap;

pub mod class;
pub mod methods;
pub mod subtyping;
pub mod terms;
pub use subtyping::is_subtype;
pub use terms::check;

pub type Env = HashMap<Var, ClassName>;
