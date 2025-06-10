use super::Value;
use crate::{terms::Raise as RaiseT, types::Type};
use common::errors::ValueKind;
use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Raise<V, Ty>
where
    V: Value,
    Ty: Type,
{
    pub val: Box<V>,
    pub cont_ty: Ty,
    pub exception_ty: Ty,
}

impl<V, Ty> Raise<V, Ty>
where
    V: Value,
    Ty: Type,
{
    pub fn new<V1, Ty1, Ty2>(v: V1, cont_ty: Ty1, ex_ty: Ty2) -> Raise<V, Ty>
    where
        V1: Into<V>,
        Ty1: Into<Ty>,
        Ty2: Into<Ty>,
    {
        Raise {
            val: Box::new(v.into()),
            cont_ty: cont_ty.into(),
            exception_ty: ex_ty.into(),
        }
    }
}

impl<V, Ty> Value for Raise<V, Ty>
where
    V: Value,
    Ty: Type,
{
    type Term = RaiseT<<V as Value>::Term, Ty>;

    fn knd(&self) -> ValueKind {
        ValueKind::Raise
    }
}

impl<V, Ty> From<Raise<V, Ty>> for RaiseT<<V as Value>::Term, Ty>
where
    V: Value,
    Ty: Type,
{
    fn from(r: Raise<V, Ty>) -> RaiseT<<V as Value>::Term, Ty> {
        RaiseT::new(*r.val, r.exception_ty, r.cont_ty)
    }
}

impl<V, Ty> fmt::Display for Raise<V, Ty>
where
    V: Value,
    Ty: Type,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "raise[{}]({} : {})",
            self.cont_ty, self.val, self.exception_ty
        )
    }
}
