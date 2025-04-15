use super::Value;
use crate::{
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
