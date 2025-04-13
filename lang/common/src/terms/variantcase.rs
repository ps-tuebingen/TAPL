use super::Term;
use crate::{Label, Var};

pub struct VariantCase<T>
where
    T: Term,
{
    bound_term: Box<T>,
    patterms: Vec<VariantPattern<T>>,
}

pub struct VariantPattern<T>
where
    T: Term,
{
    label: Label,
    bound_var: Var,
    rhs: Box<T>,
}

impl<T> Term for VariantCase<T> where T: Term {}
