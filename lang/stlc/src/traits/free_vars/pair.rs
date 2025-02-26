use super::FreeVars;
use crate::{
    syntax::{Pair, Proj1, Proj2},
    Var,
};
use std::collections::HashSet;

impl FreeVars for Pair {
    fn free_vars(&self) -> HashSet<Var> {
        let mut vars = self.fst.free_vars();
        vars.extend(self.snd.free_vars());
        vars
    }
}

impl FreeVars for Proj1 {
    fn free_vars(&self) -> HashSet<Var> {
        self.pair.free_vars()
    }
}

impl FreeVars for Proj2 {
    fn free_vars(&self) -> HashSet<Var> {
        self.pair.free_vars()
    }
}
