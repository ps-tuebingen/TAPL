use super::FreeVars;
use crate::{
    syntax::{Variant, VariantCase, VariantPattern},
    Var,
};
use std::collections::HashSet;

impl FreeVars for Variant {
    fn free_vars(&self) -> HashSet<Var> {
        self.term.free_vars()
    }
}

impl FreeVars for VariantCase {
    fn free_vars(&self) -> HashSet<Var> {
        let mut vars = self.bound_term.free_vars();
        for pt in self.cases.iter() {
            vars.extend(pt.free_vars())
        }
        vars
    }
}

impl FreeVars for VariantPattern {
    fn free_vars(&self) -> HashSet<Var> {
        let mut vars = self.rhs.free_vars();
        vars.remove(&self.bound_var);
        vars
    }
}
