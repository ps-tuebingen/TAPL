use super::Value;
use crate::{
    errors::ErrorKind,
    terms::{Raise as RaiseT, Term},
    types::Type,
};
use std::{
    any::{type_name_of_val, Any},
    fmt,
    marker::PhantomData,
};

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

impl<V, Ty, T> Raise<V, Ty, T>
where
    V: Value<T>,
    T: Term,
    Ty: Type,
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

impl<V, Ty, T> Value<T> for Raise<V, Ty, T>
where
    V: Value<T> + Into<T>,
    Ty: Type,
    T: Term + From<RaiseT<T, Ty>>,
{
    type Term = RaiseT<T, Ty>;

    fn into_raise<V1, Ty1>(self) -> Result<Raise<V1, Ty1, T>, ErrorKind>
    where
        Ty1: Type,
        V1: Value<T>,
    {
        let boxed = Box::new(self) as Box<dyn Any>;
        boxed.try_into()
    }
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

impl<V, Ty, T> TryFrom<Box<dyn Any>> for Raise<V, Ty, T>
where
    Ty: Type,
    T: Term,
    V: Value<T>,
{
    type Error = ErrorKind;
    fn try_from(boxed: Box<dyn Any>) -> Result<Raise<V, Ty, T>, Self::Error> {
        let ty_name = type_name_of_val(&(*boxed)).to_owned();
        boxed
            .downcast::<Raise<V, Ty, T>>()
            .map_err(|_| ErrorKind::TypeMismatch {
                found: ty_name,
                expected: "Raise".to_owned(),
            })
            .map(|fun| *fun)
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
