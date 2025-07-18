use super::Value;
use crate::{
    Var,
    terms::{Term, UntypedLambda as UntypedLambdaT},
};
use errors::ValueKind;
use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct UntypedLambda<T>
where
    T: Term,
{
    pub var: Var,
    pub body: T,
}

impl<T> UntypedLambda<T>
where
    T: Term,
{
    pub fn new<T1>(v: &str, bd: T1) -> UntypedLambda<T>
    where
        T1: Into<T>,
    {
        UntypedLambda {
            var: v.to_owned(),
            body: bd.into(),
        }
    }
}

impl<T> Value for UntypedLambda<T>
where
    T: Term,
{
    type Term = UntypedLambdaT<T>;

    fn knd(&self) -> ValueKind {
        ValueKind::Lambda
    }
}

impl<T> From<UntypedLambda<T>> for UntypedLambdaT<T>
where
    T: Term,
{
    fn from(lam: UntypedLambda<T>) -> UntypedLambdaT<T> {
        UntypedLambdaT::new(&lam.var, lam.body)
    }
}

impl<T> fmt::Display for UntypedLambda<T>
where
    T: Term,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\\{}.{}", self.var, self.body)
    }
}
