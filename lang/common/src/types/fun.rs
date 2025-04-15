use super::Type;
use crate::{errors::ErrorKind, subst::SubstType, TypeVar};
use std::{
    any::{type_name_of_val, Any},
    fmt,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Fun<Ty>
where
    Ty: Type,
{
    pub from: Box<Ty>,
    pub to: Box<Ty>,
}

impl<Ty> Fun<Ty>
where
    Ty: Type,
{
    pub fn new<T: Into<Ty>, U: Into<Ty>>(from: T, to: U) -> Fun<Ty> {
        Fun {
            from: Box::new(from.into()),
            to: Box::new(to.into()),
        }
    }
}

impl<Ty> Type for Fun<Ty>
where
    Ty: Type,
{
    fn into_fun<Ty1>(self) -> Result<Fun<Ty1>, ErrorKind>
    where
        Ty1: Type,
    {
        let boxed = Box::new(self) as Box<dyn Any>;
        boxed.try_into()
    }
}

impl<Ty> SubstType<Ty> for Fun<Ty>
where
    Ty: Type + SubstType<Ty, Target = Ty>,
    Self: Into<Ty>,
{
    type Target = Ty;
    fn subst_type(self, v: &TypeVar, ty: &Ty) -> Self::Target {
        Fun {
            from: Box::new(self.from.subst_type(v, ty)),
            to: Box::new(self.to.subst_type(v, ty)),
        }
        .into()
    }
}

impl<Ty> TryFrom<Box<dyn Any>> for Fun<Ty>
where
    Ty: Type,
{
    type Error = ErrorKind;
    fn try_from(boxed: Box<dyn Any>) -> Result<Fun<Ty>, Self::Error> {
        let ty_name = type_name_of_val(&(*boxed)).to_owned();
        boxed
            .downcast::<Fun<Ty>>()
            .map_err(|_| ErrorKind::TypeMismatch {
                found: ty_name,
                expected: "Function Type".to_owned(),
            })
            .map(|fun| *fun)
    }
}

impl<Ty> fmt::Display for Fun<Ty>
where
    Ty: Type,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} -> {}", self.from, self.to)
    }
}
