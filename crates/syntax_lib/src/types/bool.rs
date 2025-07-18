use super::Type;
use crate::{TypeVar, subst::SubstType};
use errors::TypeKind;
use std::{fmt, marker::PhantomData};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Bool<Ty>
where
    Ty: Type,
{
    phantom: PhantomData<Ty>,
}

impl<Ty> Bool<Ty>
where
    Ty: Type,
{
    pub fn new() -> Bool<Ty> {
        Bool {
            phantom: PhantomData,
        }
    }
}

impl<Ty> Default for Bool<Ty>
where
    Ty: Type,
{
    fn default() -> Bool<Ty> {
        Bool::new()
    }
}

impl<Ty> Type for Bool<Ty>
where
    Ty: Type,
{
    fn knd(&self) -> TypeKind {
        TypeKind::Bool
    }
}

impl<Ty> SubstType<Ty> for Bool<Ty>
where
    Ty: Type,
    Self: Into<Ty>,
{
    type Target = Ty;
    fn subst_type(self, _: &TypeVar, _: &Ty) -> Self::Target {
        self.into()
    }
}

impl<Ty> fmt::Display for Bool<Ty>
where
    Ty: Type,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Bool")
    }
}
