use super::Value;
use crate::{
    terms::{Lambda as LambdaT, Term},
    types::Type,
    Var,
};
use std::fmt;

#[derive(Debug)]
pub struct Lambda<T, Ty>
where
    T: Term,
    Ty: Type,
{
    pub var: Var,
    pub annot: Ty,
    pub body: T,
}

impl<T, Ty> Value<T> for Lambda<T, Ty>
where
    T: Term + From<LambdaT<T, Ty>>,
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

impl<T, Ty> fmt::Display for Lambda<T, Ty>
where
    T: Term,
    Ty: Type,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\\{}:{}.{}", self.var, self.annot, self.body)
    }
}
