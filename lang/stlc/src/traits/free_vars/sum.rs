use super::FreeVars;
use crate::syntax::{Left, Right, SumCase};
use common::Var;
use std::collections::HashSet;

impl FreeVars for Left {
    fn free_vars(&self) -> HashSet<Var> {
        self.left_term.free_vars()
    }
}

impl FreeVars for Right {
    fn free_vars(&self) -> HashSet<Var> {
        self.right_term.free_vars()
    }
}

impl FreeVars for SumCase {
    fn free_vars(&self) -> HashSet<Var> {
        let mut vars = self.bound_term.free_vars();
        let mut left_vars = self.left_term.free_vars();
        left_vars.remove(&self.left_var);
        vars.extend(left_vars);
        let mut right_vars = self.right_term.free_vars();
        right_vars.remove(&self.right_var);
        vars.extend(right_vars);
        vars
    }
}
