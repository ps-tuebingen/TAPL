use super::{Lambda, Raise, Value};
use crate::{
    errors::ErrorKind,
    kinds::Kind,
    terms::{Term, TyLambda as TyLambdaT},
    types::Type,
    TypeVar,
};
use std::fmt;
#[derive(Debug, PartialEq, Eq, Clone)]
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
    fn into_lambda<Ty1>(self) -> Result<Lambda<T, Ty1>, ErrorKind>
    where
        Ty1: Type,
    {
        Err(ErrorKind::TypeMismatch {
            found: self.to_string(),
            expected: "Lambda Abstraction".to_owned(),
        })
    }

    fn into_raise<Val, Ty1>(self) -> Result<Raise<Val, Ty1, T>, ErrorKind>
    where
        Val: Value<T>,
        Ty1: Type,
    {
        Err(ErrorKind::TypeMismatch {
            found: self.to_string(),
            expected: "Raise".to_owned(),
        })
    }
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
