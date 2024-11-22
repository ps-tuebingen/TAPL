use super::FreeVars;
use crate::{
    terms::syntax::{Proj, Tup},
    Var,
};
use std::collections::HashSet;

impl FreeVars for Tup {
    fn free_vars(&self) -> HashSet<Var> {
        let mut vars = HashSet::new();
        let _ = self.terms.iter().map(|t| vars.extend(t.free_vars()));
        vars
    }
}

impl FreeVars for Proj {
    fn free_vars(&self) -> HashSet<Var> {
        self.tup.free_vars()
    }
}
