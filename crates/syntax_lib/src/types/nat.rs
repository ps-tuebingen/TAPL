use super::Type;
use crate::{TypeVar, subst::SubstType};
use common::errors::TypeKind;
use std::{fmt, marker::PhantomData};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Nat<Ty>
where
    Ty: Type,
{
    phantom: PhantomData<Ty>,
}

impl<Ty> Nat<Ty>
where
    Ty: Type,
{
    pub fn new() -> Nat<Ty> {
        Nat {
            phantom: PhantomData,
        }
    }
}

impl<Ty> Default for Nat<Ty>
where
    Ty: Type,
{
    fn default() -> Nat<Ty> {
        Nat::new()
    }
}

impl<Ty> Type for Nat<Ty>
where
    Ty: Type,
{
    fn knd(&self) -> TypeKind {
        TypeKind::Nat
    }
}

impl<Ty> SubstType<Ty> for Nat<Ty>
where
    Ty: Type + SubstType<Ty, Target = Ty>,
    Self: Into<Ty>,
{
    type Target = Ty;
    fn subst_type(self, _: &TypeVar, _: &Ty) -> Self::Target {
        self.into()
    }
}

impl<Ty> fmt::Display for Nat<Ty>
where
    Ty: Type,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Nat")
    }
}
