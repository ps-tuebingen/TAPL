use super::Value;
use crate::{
    kinds::Kind,
    terms::{Term, TyLambda as TyLambdaT},
    TypeVar,
};
use std::fmt;
#[derive(Debug)]
pub struct TyLambda<T>
where
    T: Term,
{
    var: TypeVar,
    annot: Kind,
    term: T,
}

impl<T> Value<T> for TyLambda<T>
where
    T: Term + From<TyLambdaT<T>>,
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
        write!(f, "\\{}.{}", self.var, self.term)
    }
}
