use super::Value;
use std::{fmt, marker::PhantomData};
use syntax::{
    terms::{Exception as ExceptionT, Term},
    types::Type,
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Exception<T, Ty>
where
    Ty: Type,
    T: Term,
{
    ty: Ty,
    phantom: PhantomData<T>,
}

impl<T, Ty> Exception<T, Ty>
where
    Ty: Type,
    T: Term,
{
    pub fn new<Ty1>(ty: Ty1) -> Exception<T, Ty>
    where
        Ty1: Into<Ty>,
    {
        Exception {
            ty: ty.into(),
            phantom: PhantomData,
        }
    }
}

impl<T, Ty> Value for Exception<T, Ty>
where
    T: Term,
    Ty: Type,
{
    type Term = ExceptionT<T, Ty>;
}

impl<T, Ty> From<Exception<T, Ty>> for ExceptionT<T, Ty>
where
    T: Term,
    Ty: Type,
{
    fn from(ex: Exception<T, Ty>) -> ExceptionT<T, Ty> {
        ExceptionT::new(ex.ty)
    }
}

impl<T, Ty> fmt::Display for Exception<T, Ty>
where
    T: Term,
    Ty: Type,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "error[{}]", self.ty)
    }
}
