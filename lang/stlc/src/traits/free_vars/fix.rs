use super::FreeVars;
use crate::syntax::Fix;
use common::Var;
use std::collections::HashSet;

impl FreeVars for Fix {
    fn free_vars(&self) -> HashSet<Var> {
        self.term.free_vars()
    }
}
