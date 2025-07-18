use super::Value;
use crate::{terms::Pack as PackT, types::Type};
use errors::ValueKind;
use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Pack<V, Ty>
where
    V: Value,
    Ty: Type,
{
    pub inner_ty: Ty,
    pub val: Box<V>,
    pub outer_ty: Ty,
}

impl<V, Ty> Pack<V, Ty>
where
    V: Value,
    Ty: Type,
{
    pub fn new<Ty1, V1, Ty2>(inner: Ty1, v: V1, outer: Ty2) -> Pack<V, Ty>
    where
        Ty1: Into<Ty>,
        Ty2: Into<Ty>,
        V1: Into<V>,
    {
        Pack {
            inner_ty: inner.into(),
            val: Box::new(v.into()),
            outer_ty: outer.into(),
        }
    }
}

impl<V, Ty> Value for Pack<V, Ty>
where
    V: Value,
    Ty: Type,
{
    type Term = PackT<<V as Value>::Term, Ty>;

    fn knd(&self) -> ValueKind {
        ValueKind::Package
    }
}

impl<V, Ty> From<Pack<V, Ty>> for PackT<<V as Value>::Term, Ty>
where
    V: Value,
    Ty: Type,
{
    fn from(pack: Pack<V, Ty>) -> PackT<<V as Value>::Term, Ty> {
        PackT::new(pack.inner_ty, *pack.val, pack.outer_ty)
    }
}

impl<V, Ty> fmt::Display for Pack<V, Ty>
where
    V: Value,
    Ty: Type,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{{*{},{}}} as {}",
            self.inner_ty, self.val, self.outer_ty
        )
    }
}
