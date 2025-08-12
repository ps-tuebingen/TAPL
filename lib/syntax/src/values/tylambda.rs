use super::Value;
use crate::{
    TypeVar,
    kinds::Kind,
    terms::{Term, TyLambda as TyLambdaT},
};
use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TyLambda<T>
where
    T: Term,
{
    pub var: TypeVar,
    pub annot: Kind,
    pub term: T,
}

impl<T> TyLambda<T>
where
    T: Term,
{
    pub fn new<T1>(v: &str, knd: Kind, t: T1) -> TyLambda<T>
    where
        T1: Into<T>,
    {
        TyLambda {
            var: v.to_owned(),
            annot: knd,
            term: t.into(),
        }
    }
}

impl<T> Value for TyLambda<T>
where
    T: Term,
{
    type Term = TyLambdaT<T>;
}
impl<T> From<TyLambda<T>> for TyLambdaT<T>
where
    T: Term,
{
    fn from(tylam: TyLambda<T>) -> TyLambdaT<T> {
        TyLambdaT::new(&tylam.var, tylam.annot, tylam.term)
    }
}

impl<T> fmt::Display for TyLambda<T>
where
    T: Term,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\\{}::{}.{}", self.var, self.annot, self.term)
    }
}
