use super::{Lambda, Raise, Value};
use crate::{
    errors::ErrorKind,
    terms::{Fold as FoldT, Term},
    types::Type,
};
use std::fmt;
use std::marker::PhantomData;
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Fold<V, Ty, T>
where
    V: Value<T>,
    Ty: Type,
    T: Term,
{
    ty: Ty,
    val: Box<V>,
    phantom: PhantomData<T>,
}

impl<V, Ty, T> Value<T> for Fold<V, Ty, T>
where
    V: Value<T> + Into<T>,
    Ty: Type,
    T: Term + From<FoldT<T, Ty>>,
{
    type Term = FoldT<T, Ty>;
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

impl<V, Ty, T> From<Fold<V, Ty, T>> for FoldT<T, Ty>
where
    T: Term,
    V: Value<T> + Into<T>,
    Ty: Type,
{
    fn from(fld: Fold<V, Ty, T>) -> FoldT<T, Ty> {
        FoldT::new(*fld.val, fld.ty)
    }
}

impl<V, T, Ty> fmt::Display for Fold<V, Ty, T>
where
    V: Value<T>,
    T: Term,
    Ty: Type,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "fold[{}]({})", self.ty, self.val)
    }
}
