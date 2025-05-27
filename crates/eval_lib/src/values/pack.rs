use super::Value;
use std::{fmt, marker::PhantomData};
use syntax::{
    terms::{Pack as PackT, Term},
    types::Type,
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Pack<V, Ty, T>
where
    V: Value,
    Ty: Type,
    T: Term,
{
    pub inner_ty: Ty,
    pub val: Box<V>,
    pub outer_ty: Ty,
    phantom: PhantomData<T>,
}

impl<V, Ty, T> Pack<V, Ty, T>
where
    V: Value,
    Ty: Type,
    T: Term,
{
    pub fn new<Ty1, V1, Ty2>(inner: Ty1, v: V1, outer: Ty2) -> Pack<V, Ty, T>
    where
        Ty1: Into<Ty>,
        Ty2: Into<Ty>,
        V: Into<V>,
    {
        Pack {
            inner_ty: inner.into(),
            val: Box::new(v.into()),
            outer_ty: outer.into(),
            phantom: PhantomData,
        }
    }
}

impl<V, Ty, T> Value for Pack<V, Ty, T>
where
    V: Value,
    Ty: Type,
    T: Term,
{
    type Term = PackT<T, Ty>;
}

impl<V, Ty, T> From<Pack<V, Ty, T>> for PackT<T, Ty>
where
    T: Term,
{
    fn from(pack: Pack<V, Ty, T>) -> PackT<T, Ty> {
        PackT::new(pack.inner_ty, *pack.val, pack.outer_ty)
    }
}

impl<V, Ty, T> fmt::Display for Pack<V, Ty, T>
where
    V: Value,
    Ty: Type,
    T: Term,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{{*{},{}}} as {}",
            self.inner_ty, self.val, self.outer_ty
        )
    }
}
