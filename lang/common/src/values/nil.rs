use super::Value;
use crate::{
    terms::{Nil as NilT, Term},
    types::Type,
};
use std::fmt;
#[derive(Debug, Clone)]
pub struct Nil<Ty>
where
    Ty: Type,
{
    ty: Ty,
}

impl<Ty, T> Value<T> for Nil<Ty>
where
    Ty: Type,
    T: Term + From<NilT<T, Ty>>,
{
    type Term = NilT<T, Ty>;
}

impl<T, Ty> From<Nil<Ty>> for NilT<T, Ty>
where
    T: Term,
    Ty: Type,
{
    fn from(nil: Nil<Ty>) -> NilT<T, Ty> {
        NilT::new(nil.ty)
    }
}

impl<Ty> fmt::Display for Nil<Ty>
where
    Ty: Type,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "nil[{}]", self.ty)
    }
}
