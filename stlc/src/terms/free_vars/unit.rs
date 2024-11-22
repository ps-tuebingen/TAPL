use super::FreeVars;
use crate::{terms::syntax::Unit, Var};
use std::collections::HashSet;

impl FreeVars for Unit {
    fn free_vars(&self) -> HashSet<Var> {
        HashSet::new()
    }
}
