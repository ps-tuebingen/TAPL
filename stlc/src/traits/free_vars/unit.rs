use super::FreeVars;
use crate::{syntax::Unit, Var};
use std::collections::HashSet;

impl FreeVars for Unit {
    fn free_vars(&self) -> HashSet<Var> {
        HashSet::new()
    }
}
