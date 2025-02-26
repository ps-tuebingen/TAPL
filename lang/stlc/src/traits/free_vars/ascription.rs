use super::FreeVars;
use crate::{syntax::Ascribe, Var};
use std::collections::HashSet;

impl FreeVars for Ascribe {
    fn free_vars(&self) -> HashSet<Var> {
        self.term.free_vars()
    }
}
