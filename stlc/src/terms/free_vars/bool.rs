use super::FreeVars;
use crate::{
    terms::syntax::{False, If, True},
    Var,
};
use std::collections::HashSet;

impl FreeVars for True {
    fn free_vars(&self) -> HashSet<Var> {
        HashSet::new()
    }
}

impl FreeVars for False {
    fn free_vars(&self) -> HashSet<Var> {
        HashSet::new()
    }
}

impl FreeVars for If {
    fn free_vars(&self) -> HashSet<Var> {
        let mut vars = self.ifc.free_vars();
        vars.extend(self.thenc.free_vars());
        vars.extend(self.elsec.free_vars());
        vars
    }
}
