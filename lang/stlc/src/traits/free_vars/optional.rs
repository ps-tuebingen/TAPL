use super::FreeVars;
use crate::syntax::{Nothing, SomeCase, Something};
use common::Var;
use std::collections::HashSet;

impl FreeVars for Nothing {
    fn free_vars(&self) -> HashSet<Var> {
        HashSet::new()
    }
}

impl FreeVars for Something {
    fn free_vars(&self) -> HashSet<Var> {
        self.term.free_vars()
    }
}

impl FreeVars for SomeCase {
    fn free_vars(&self) -> HashSet<Var> {
        let mut vars = self.some_rhs.free_vars();
        vars.remove(&self.some_var);
        vars.extend(self.none_rhs.free_vars());
        vars.extend(self.bound_term.free_vars());
        vars
    }
}
