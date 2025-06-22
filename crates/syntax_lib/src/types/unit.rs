use super::Type;
use crate::{TypeVar, subst::SubstType};
use common::errors::TypeKind;
use std::{fmt, marker::PhantomData};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Unit<Ty>
where
    Ty: Type,
{
    phantom: PhantomData<Ty>,
}

impl<Ty> Unit<Ty>
where
    Ty: Type,
{
    pub fn new() -> Unit<Ty> {
        Unit {
            phantom: PhantomData,
        }
    }
}

impl<Ty> Default for Unit<Ty>
where
    Ty: Type,
{
    fn default() -> Unit<Ty> {
        Unit::new()
    }
}

impl<Ty> Type for Unit<Ty>
where
    Ty: Type,
{
    fn knd(&self) -> TypeKind {
        TypeKind::Unit
    }
}

impl<Ty> SubstType<Ty> for Unit<Ty>
where
    Ty: Type,
    Self: Into<Ty>,
{
    type Target = Ty;
    fn subst_type(self, _: &TypeVar, _: &Ty) -> Self::Target {
        self.into()
    }
}

impl<Ty> fmt::Display for Unit<Ty>
where
    Ty: Type,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Unit")
    }
}
