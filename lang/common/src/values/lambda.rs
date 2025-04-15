use super::Value;
use crate::{
    terms::{Lambda as LambdaT, Term},
    types::Type,
    Var,
};

pub struct Lambda<T, Ty>
where
    T: Term,
    Ty: Type,
{
    var: Var,
    annot: Ty,
    body: T,
}

impl<T, Ty> Value<T> for Lambda<T, Ty>
where
    T: Term,
    Ty: Type,
{
    type Term = LambdaT<T, Ty>;
}

impl<T, Ty> From<Lambda<T, Ty>> for LambdaT<T, Ty>
where
    T: Term,
    Ty: Type,
{
    fn from(lam: Lambda<T, Ty>) -> LambdaT<T, Ty> {
        LambdaT::new(&lam.var, lam.annot, lam.body)
    }
}
