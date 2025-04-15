use super::{Lambda, Raise, Value};
use crate::{
    errors::ErrorKind,
    terms::{Pack as PackT, Term},
    types::Type,
};
use std::fmt;
use std::marker::PhantomData;
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Pack<V, Ty, T>
where
    V: Value<T>,
    Ty: Type,
    T: Term,
{
    inner_ty: Ty,
    val: Box<V>,
    outer_ty: Ty,
    phantom: PhantomData<T>,
}

impl<V, Ty, T> Value<T> for Pack<V, Ty, T>
where
    V: Value<T> + Into<T>,
    Ty: Type,
    T: Term + From<PackT<T, Ty>>,
{
    type Term = PackT<T, Ty>;
    fn into_lambda<Ty1>(self) -> Result<Lambda<T, Ty1>, ErrorKind>
    where
        Ty1: Type,
    {
        Err(ErrorKind::TypeMismatch {
            found: self.to_string(),
            expected: "Lambda Abstraction".to_owned(),
        })
    }

    fn into_raise<Val, Ty1>(self) -> Result<Raise<Val, Ty1, T>, ErrorKind>
    where
        Val: Value<T>,
        Ty1: Type,
    {
        Err(ErrorKind::TypeMismatch {
            found: self.to_string(),
            expected: "Raise".to_owned(),
        })
    }
}

impl<V, Ty, T> From<Pack<V, Ty, T>> for PackT<T, Ty>
where
    T: Term,
    V: Value<T> + Into<T>,
    Ty: Type,
{
    fn from(pack: Pack<V, Ty, T>) -> PackT<T, Ty> {
        PackT::new(pack.inner_ty, *pack.val, pack.outer_ty)
    }
}

impl<V, Ty, T> fmt::Display for Pack<V, Ty, T>
where
    V: Value<T>,
    T: Term,
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
