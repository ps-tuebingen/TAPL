use crate::Var;
use std::collections::HashSet;

pub mod ascription;
pub mod bool;
pub mod fix;
pub mod lambda;
pub mod let_exp;
pub mod list;
pub mod optional;
pub mod pair;
pub mod record;
pub mod sum;
pub mod term;
pub mod tuple;
pub mod unit;
pub mod variant;

pub trait FreeVars {
    fn free_vars(&self) -> HashSet<Var>;
}

impl FreeVars for Var {
    fn free_vars(&self) -> HashSet<Var> {
        HashSet::from([self.clone()])
    }
}
