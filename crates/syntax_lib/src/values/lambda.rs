use super::Value;
use crate::{
    Var,
    terms::{Lambda as LambdaT, Term},
    types::Type,
};
use errors::ValueKind;
use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Lambda<T, Ty>
where
    T: Term,
    Ty: Type,
{
    pub var: Var,
    pub annot: Ty,
    pub body: T,
}

impl<T, Ty> Lambda<T, Ty>
where
    T: Term,
    Ty: Type,
{
    pub fn new<Ty1, T1>(v: &str, ty: Ty1, bd: T1) -> Lambda<T, Ty>
    where
        T1: Into<T>,
        Ty1: Into<Ty>,
    {
        Lambda {
            var: v.to_owned(),
            annot: ty.into(),
            body: bd.into(),
        }
    }
}

impl<T, Ty> Value for Lambda<T, Ty>
where
    T: Term,
    Ty: Type,
{
    type Term = LambdaT<T, Ty>;

    fn knd(&self) -> ValueKind {
        ValueKind::Lambda
    }
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
        let ty_str = self.annot.to_string();
        if ty_str.is_empty() {
            write!(f, "\\{}.{}", self.var, self.body)
        } else {
            write!(f, "\\{}:{}.({})", self.var, ty_str, self.body)
        }
    }
}
