use super::{TypeVar, Var};
use std::collections::HashSet;

pub trait FreeVars {
    fn free_vars(&self, vars: &mut HashSet<Var>);
}

pub trait FreeTypeVars {
    fn free_type_vars(&self, vars: &mut HashSet<TypeVar>);
}
