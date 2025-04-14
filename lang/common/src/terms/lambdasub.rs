use super::Term;
use crate::{types::Type, Var};
use std::fmt;

#[derive(Clone, Debug)]
pub struct LambdaSub<T, Ty>
where
    T: Term,
    Ty: Type,
{
    var: Var,
    sup_ty: Ty,
    body: Box<T>,
}

impl<T, Ty> fmt::Display for LambdaSub<T, Ty>
where
    T: Term,
    Ty: Type,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\\{}<:{}.{}", self.var, self.sup_ty, self.body)
    }
}
