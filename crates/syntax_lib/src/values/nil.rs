use super::Value;
use crate::{
    terms::{Nil as NilT, Term},
    types::Type,
};
use common::errors::ValueKind;
use std::{fmt, marker::PhantomData};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Nil<T, Ty>
where
    T: Term,
    Ty: Type,
{
    pub ty: Ty,
    phantom: PhantomData<T>,
}

impl<T, Ty> Nil<T, Ty>
where
    T: Term,
    Ty: Type,
{
    pub fn new<Typ>(ty: Typ) -> Nil<T, Ty>
    where
        Typ: Into<Ty>,
    {
        Nil {
            ty: ty.into(),
            phantom: PhantomData,
        }
    }
}

impl<T, Ty> Value for Nil<T, Ty>
where
    T: Term,
    Ty: Type,
{
    type Term = NilT<T, Ty>;

    fn knd(&self) -> ValueKind {
        ValueKind::Nil
    }
}

impl<T, Ty> From<Nil<T, Ty>> for NilT<T, Ty>
where
    T: Term,
    Ty: Type,
{
    fn from(nil: Nil<T, Ty>) -> NilT<T, Ty> {
        NilT::new(nil.ty)
    }
}

impl<T, Ty> fmt::Display for Nil<T, Ty>
where
    T: Term,
    Ty: Type,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Nil[{}]", self.ty)
    }
}
