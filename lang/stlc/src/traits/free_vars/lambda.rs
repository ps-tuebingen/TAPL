use super::FreeVars;
use crate::syntax::{App, Lambda};
use common::Var;
use std::collections::HashSet;

impl FreeVars for Lambda {
    fn free_vars(&self) -> HashSet<Var> {
        let mut vars = self.body.free_vars();
        vars.remove(&self.var);
        vars
    }
}

impl FreeVars for App {
    fn free_vars(&self) -> HashSet<Var> {
        let mut vars = self.fun.free_vars();
        vars.extend(self.arg.free_vars());
        vars
    }
}
