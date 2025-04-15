use super::Value;
use crate::{
    terms::{Raise as RaiseT, Term},
    types::Type,
};
use std::fmt;
use std::marker::PhantomData;
#[derive(Debug, Clone)]
pub struct Raise<V, Ty, T>
where
    V: Value<T>,
    Ty: Type,
    T: Term,
{
    val: Box<V>,
    cont_ty: Ty,
    exception_ty: Ty,
    phantom: PhantomData<T>,
}

impl<V, Ty, T> Value<T> for Raise<V, Ty, T>
where
    V: Value<T> + Into<T>,
    Ty: Type,
    T: Term + From<RaiseT<T, Ty>>,
{
    type Term = RaiseT<T, Ty>;
}

impl<V, Ty, T> From<Raise<V, Ty, T>> for RaiseT<T, Ty>
where
    V: Value<T> + Into<T>,
    T: Term,
    Ty: Type,
{
    fn from(r: Raise<V, Ty, T>) -> RaiseT<T, Ty> {
        RaiseT::new(*r.val, r.exception_ty, r.cont_ty)
    }
}

impl<V, Ty, T> fmt::Display for Raise<V, Ty, T>
where
    V: Value<T>,
    T: Term,
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
