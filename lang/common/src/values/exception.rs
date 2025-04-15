use super::Value;
use crate::{
    terms::{Exception as ExceptionT, Term},
    types::Type,
};
use std::fmt;

#[derive(Debug)]
pub struct Exception<Ty>
where
    Ty: Type,
{
    ty: Ty,
}

impl<T, Ty> Value<T> for Exception<Ty>
where
    Ty: Type,
    T: Term + From<ExceptionT<T, Ty>>,
{
    type Term = ExceptionT<T, Ty>;
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
