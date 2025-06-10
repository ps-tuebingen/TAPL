use super::Value;
use crate::{
    terms::{LambdaSub as LambdaSubT, Term},
    types::Type,
    Var,
};
use common::errors::ValueKind;
use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct LambdaSub<T, Ty>
where
    T: Term,
    Ty: Type,
{
    pub var: Var,
    pub sup_ty: Ty,
    pub term: T,
}

impl<T, Ty> LambdaSub<T, Ty>
where
    T: Term,
    Ty: Type,
{
    pub fn new<Ty1, T1>(v: &str, sup: Ty1, t: T1) -> LambdaSub<T, Ty>
    where
        Ty1: Into<Ty>,
        T1: Into<T>,
    {
        LambdaSub {
            var: v.to_owned(),
            sup_ty: sup.into(),
            term: t.into(),
        }
    }
}

impl<T, Ty> Value for LambdaSub<T, Ty>
where
    T: Term,
    Ty: Type,
{
    type Term = LambdaSubT<T, Ty>;

    fn knd(&self) -> ValueKind {
        ValueKind::LambdaSub
    }
}

impl<T, Ty> From<LambdaSub<T, Ty>> for LambdaSubT<T, Ty>
where
    T: Term,
    Ty: Type,
{
    fn from(lam: LambdaSub<T, Ty>) -> LambdaSubT<T, Ty> {
        LambdaSubT::new(&lam.var, lam.sup_ty, lam.term)
    }
}

impl<T, Ty> fmt::Display for LambdaSub<T, Ty>
where
    T: Term,
    Ty: Type,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\\{}<:({}).{}", self.var, self.sup_ty, self.term)
    }
}
