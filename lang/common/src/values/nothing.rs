use super::{Lambda, Raise, Value};
use crate::{
    errors::ErrorKind,
    terms::{Nothing as NothingT, Term},
    types::Type,
};
use std::fmt;
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Nothing<Ty>
where
    Ty: Type,
{
    ty: Ty,
}

impl<Ty, T> Value<T> for Nothing<Ty>
where
    Ty: Type,
    T: Term + From<NothingT<T, Ty>>,
{
    type Term = NothingT<T, Ty>;
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

impl<T, Ty> From<Nothing<Ty>> for NothingT<T, Ty>
where
    T: Term,
    Ty: Type,
{
    fn from(not: Nothing<Ty>) -> NothingT<T, Ty> {
        NothingT::new(not.ty)
    }
}

impl<Ty> fmt::Display for Nothing<Ty>
where
    Ty: Type,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "nothing[{}]", self.ty)
    }
}
