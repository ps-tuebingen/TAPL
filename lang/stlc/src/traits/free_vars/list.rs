use super::FreeVars;
use crate::{
    syntax::{Cons, Head, IsNil, Nil, Tail},
    Var,
};
use std::collections::HashSet;

impl FreeVars for Nil {
    fn free_vars(&self) -> HashSet<Var> {
        HashSet::new()
    }
}

impl FreeVars for Cons {
    fn free_vars(&self) -> HashSet<Var> {
        let mut vars = self.fst.free_vars();
        vars.extend(self.rst.free_vars());
        vars
    }
}

impl FreeVars for IsNil {
    fn free_vars(&self) -> HashSet<Var> {
        self.list.free_vars()
    }
}

impl FreeVars for Head {
    fn free_vars(&self) -> HashSet<Var> {
        self.list.free_vars()
    }
}

impl FreeVars for Tail {
    fn free_vars(&self) -> HashSet<Var> {
        self.list.free_vars()
    }
}
