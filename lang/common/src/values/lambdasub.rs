use super::Value;
use crate::{
    terms::{LambdaSub as LambdaSubT, Term},
    types::Type,
    Var,
};

pub struct LambdaSub<T, Ty>
where
    T: Term,
    Ty: Type,
{
    var: Var,
    ty: Ty,
    t: T,
}

impl<T, Ty> Value<T> for LambdaSub<T, Ty>
where
    T: Term,
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
        LambdaSubT::new(&lam.var, lam.ty, lam.t)
    }
}
