use super::FreeVars;
use crate::syntax::Let;
use common::Var;
use std::collections::HashSet;

impl FreeVars for Let {
    fn free_vars(&self) -> HashSet<Var> {
        let mut vars = self.in_term.free_vars();
        vars.remove(&self.var);
        vars.extend(self.bound_term.free_vars());
        vars
    }
}
