use super::Value;
use crate::{
    terms::{Nothing as NothingT, Term},
    types::Type,
};
use common::errors::ValueKind;
use std::{fmt, marker::PhantomData};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Nothing<T, Ty>
where
    T: Term,
    Ty: Type,
{
    pub ty: Ty,
    phantom: PhantomData<T>,
}

impl<T, Ty> Nothing<T, Ty>
where
    T: Term,
    Ty: Type,
{
    pub fn new<Typ>(ty: Typ) -> Nothing<T, Ty>
    where
        Typ: Into<Ty>,
    {
        Nothing {
            ty: ty.into(),
            phantom: PhantomData,
        }
    }
}

impl<T, Ty> Value for Nothing<T, Ty>
where
    T: Term,
    Ty: Type,
{
    type Term = NothingT<T, Ty>;

    fn knd(&self) -> ValueKind {
        ValueKind::Nothing
    }
}

impl<T, Ty> From<Nothing<T, Ty>> for NothingT<T, Ty>
where
    T: Term,
    Ty: Type,
{
    fn from(not: Nothing<T, Ty>) -> NothingT<T, Ty> {
        NothingT::new(not.ty)
    }
}

impl<T, Ty> fmt::Display for Nothing<T, Ty>
where
    T: Term,
    Ty: Type,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Nothing[{}]", self.ty)
    }
}
