use super::{Lambda, Raise, Value};
use crate::{
    errors::ErrorKind,
    terms::{Exception as ExceptionT, Term},
    types::Type,
};
use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Exception<Ty>
where
    Ty: Type,
{
    ty: Ty,
}

impl<Ty> Exception<Ty>
where
    Ty: Type,
{
    pub fn new<Ty1>(ty: Ty1) -> Exception<Ty>
    where
        Ty1: Into<Ty>,
    {
        Exception { ty: ty.into() }
    }
}

impl<T, Ty> Value<T> for Exception<Ty>
where
    Ty: Type,
    T: Term + From<ExceptionT<T, Ty>>,
{
    type Term = ExceptionT<T, Ty>;
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

impl<T, Ty> From<Exception<Ty>> for ExceptionT<T, Ty>
where
    T: Term,
    Ty: Type,
{
    fn from(ex: Exception<Ty>) -> ExceptionT<T, Ty> {
        ExceptionT::new(ex.ty)
    }
}

impl<Ty> fmt::Display for Exception<Ty>
where
    Ty: Type,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "error[{}]", self.ty)
    }
}
