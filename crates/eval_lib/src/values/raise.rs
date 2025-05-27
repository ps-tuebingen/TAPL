use super::Value;
use std::{fmt, marker::PhantomData};
use syntax::{
    terms::{Raise as RaiseT, Term},
    types::Type,
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Raise<V, Ty, T>
where
    V: Value,
    Ty: Type,
    T: Term,
{
    pub val: Box<V>,
    cont_ty: Ty,
    exception_ty: Ty,
    phantom: PhantomData<T>,
}

impl<V, Ty, T> Raise<V, Ty, T>
where
    V: Value,
    Ty: Type,
    T: Term,
{
    pub fn new<V1, Ty1, Ty2>(v: V1, cont_ty: Ty1, ex_ty: Ty2) -> Raise<V, Ty, T>
    where
        V1: Into<V>,
        Ty1: Into<Ty>,
        Ty2: Into<Ty>,
    {
        Raise {
            val: Box::new(v.into()),
            cont_ty: cont_ty.into(),
            exception_ty: ex_ty.into(),
            phantom: PhantomData,
        }
    }
}

impl<V, Ty, T> Value for Raise<V, Ty, T>
where
    V: Value,
    Ty: Type,
    T: Term,
{
    type Term = RaiseT<T, Ty>;
}

impl<V, Ty, T> From<Raise<V, Ty, T>> for RaiseT<T, Ty>
where
    V: Value,
    Ty: Type,
    T: Term,
{
    fn from(r: Raise<V, Ty, T>) -> RaiseT<T, Ty> {
        RaiseT::new(*r.val, r.exception_ty, r.cont_ty)
    }
}

impl<V, Ty, T> fmt::Display for Raise<V, Ty, T>
where
    V: Value,
    Ty: Type,
    T: Term,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "raise[{}]({} : {})",
            self.cont_ty, self.val, self.exception_ty
        )
    }
}
