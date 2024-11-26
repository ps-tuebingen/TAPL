use super::FreeVars;
use crate::{
    syntax::{Nothing, Something},
    Var,
};
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
