use super::Value;
use crate::{
    terms::{Nothing as NothingT, Term},
    types::Type,
};

pub struct Nothing<Ty>
where
    Ty: Type,
{
    ty: Ty,
}

impl<Ty, T> Value<T> for Nothing<Ty>
where
    Ty: Type,
    T: Term,
{
    type Term = NothingT<T, Ty>;
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
