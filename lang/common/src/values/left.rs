use super::{Lambda, Raise, Value};
use crate::{
    errors::ErrorKind,
    terms::{Left as LeftT, Term},
    types::Type,
};
use std::fmt;
use std::marker::PhantomData;
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Left<V, Ty, T>
where
    V: Value<T>,
    Ty: Type,
    T: Term,
{
    left_val: V,
    ty: Ty,
    phantom: PhantomData<T>,
}

impl<V, Ty, T> Value<T> for Left<V, Ty, T>
where
    V: Value<T> + Into<T>,
    Ty: Type,
    T: Term + From<LeftT<T, Ty>>,
{
    type Term = LeftT<T, Ty>;
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

impl<V, Ty, T> From<Left<V, Ty, T>> for LeftT<T, Ty>
where
    V: Value<T> + Into<T>,
    T: Term,
    Ty: Type,
{
    fn from(lft: Left<V, Ty, T>) -> LeftT<T, Ty> {
        LeftT::new(lft.left_val, lft.ty)
    }
}

impl<V, Ty, T> fmt::Display for Left<V, Ty, T>
where
    V: Value<T>,
    T: Term,
    Ty: Type,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "inl({}) as {}", self.left_val, self.ty)
    }
}
