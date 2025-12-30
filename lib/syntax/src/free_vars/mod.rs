use super::{TypeVar, Var};
use std::collections::HashSet;

pub trait FreeVars {
    fn free_vars(&self, vars: &mut HashSet<Var>);
}

pub trait FreeTypeVars {
    fn free_type_vars(&self, vars: &mut HashSet<TypeVar>);
}

impl<T> FreeTypeVars for &[T]
where
    T: FreeTypeVars,
{
    fn free_type_vars(&self, vars: &mut HashSet<TypeVar>) {
        for t in *self {
            t.free_type_vars(vars);
        }
    }
}

impl<T> FreeTypeVars for Box<T>
where
    T: FreeTypeVars,
{
    fn free_type_vars(&self, vars: &mut HashSet<TypeVar>) {
        self.as_ref().free_type_vars(vars);
    }
}
