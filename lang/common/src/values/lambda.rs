use super::Value;
use crate::{
    errors::ErrorKind,
    terms::{Lambda as LambdaT, Term},
    types::Type,
    Var,
};
use std::{
    any::{type_name_of_val, Any},
    fmt,
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Lambda<T, Ty>
where
    T: Term,
    Ty: Type,
{
    pub var: Var,
    pub annot: Ty,
    pub body: T,
}

impl<T, Ty> Lambda<T, Ty>
where
    T: Term,
    Ty: Type,
{
    pub fn new<Ty1, T1>(v: &str, ty: Ty1, bd: T1) -> Lambda<T, Ty>
    where
        T1: Into<T>,
        Ty1: Into<Ty>,
    {
        Lambda {
            var: v.to_owned(),
            annot: ty.into(),
            body: bd.into(),
        }
    }
}

impl<T, Ty> Value<T> for Lambda<T, Ty>
where
    T: Term + From<LambdaT<T, Ty>>,
    Ty: Type,
{
    type Term = LambdaT<T, Ty>;

    fn into_lambda<Ty1>(self) -> Result<Lambda<T, Ty1>, ErrorKind>
    where
        Ty1: Type,
    {
        let boxed = Box::new(self) as Box<dyn Any>;
        boxed.try_into()
    }
}

impl<T, Ty> From<Lambda<T, Ty>> for LambdaT<T, Ty>
where
    T: Term,
    Ty: Type,
{
    fn from(lam: Lambda<T, Ty>) -> LambdaT<T, Ty> {
        LambdaT::new(&lam.var, lam.annot, lam.body)
    }
}

impl<T, Ty> TryFrom<Box<dyn Any>> for Lambda<T, Ty>
where
    T: Term,
    Ty: Type,
{
    type Error = ErrorKind;
    fn try_from(boxed: Box<dyn Any>) -> Result<Lambda<T, Ty>, Self::Error> {
        let ty_name = type_name_of_val(&(*boxed)).to_owned();
        boxed
            .downcast::<Lambda<T, Ty>>()
            .map_err(|_| ErrorKind::TypeMismatch {
                found: ty_name,
                expected: "Lambda Abstraction".to_owned(),
            })
            .map(|lam| *lam)
    }
}

impl<T, Ty> fmt::Display for Lambda<T, Ty>
where
    T: Term,
    Ty: Type,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\\{}:{}.{}", self.var, self.annot, self.body)
    }
}
