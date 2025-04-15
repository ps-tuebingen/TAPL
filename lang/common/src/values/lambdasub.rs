use super::Value;
use crate::{
    terms::{LambdaSub as LambdaSubT, Term},
    types::Type,
    Var,
};
use std::fmt;
#[derive(Debug, Clone)]
pub struct LambdaSub<T, Ty>
where
    T: Term,
    Ty: Type,
{
    var: Var,
    sup_ty: Ty,
    t: T,
}

impl<T, Ty> Value<T> for LambdaSub<T, Ty>
where
    T: Term + From<LambdaSubT<T, Ty>>,
    Ty: Type,
{
    type Term = LambdaSubT<T, Ty>;
}

impl<T, Ty> From<LambdaSub<T, Ty>> for LambdaSubT<T, Ty>
where
    T: Term,
    Ty: Type,
{
    fn from(lam: LambdaSub<T, Ty>) -> LambdaSubT<T, Ty> {
        LambdaSubT::new(&lam.var, lam.sup_ty, lam.t)
    }
}

impl<T, Ty> fmt::Display for LambdaSub<T, Ty>
where
    T: Term,
    Ty: Type,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\\{}<:{}.{}", self.var, self.sup_ty, self.t)
    }
}
