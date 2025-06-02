use super::Type;
use crate::{subst::SubstType, TypeVar};
use common::errors::TypeKind;
use std::{fmt, marker::PhantomData};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TypeVariable<Ty>
where
    Ty: Type,
{
    pub v: TypeVar,
    phantom: PhantomData<Ty>,
}

impl<Ty> TypeVariable<Ty>
where
    Ty: Type,
{
    pub fn new(v: &str) -> TypeVariable<Ty> {
        TypeVariable {
            v: v.to_owned(),
            phantom: PhantomData,
        }
    }
}

impl<Ty> Type for TypeVariable<Ty>
where
    Ty: Type,
{
    fn knd(&self) -> TypeKind {
        TypeKind::Variable
    }
}

impl<Ty> SubstType<Ty> for TypeVariable<Ty>
where
    Ty: Type,
    Self: Into<Ty>,
{
    type Target = Ty;
    fn subst_type(self, v: &TypeVar, ty: &Ty) -> Self::Target {
        if *v == self.v {
            ty.clone()
        } else {
            self.into()
        }
    }
}

impl<Ty> fmt::Display for TypeVariable<Ty>
where
    Ty: Type,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.v)
    }
}
