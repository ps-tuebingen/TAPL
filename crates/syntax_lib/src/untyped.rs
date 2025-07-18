use crate::{
    TypeVar,
    subst::SubstType,
    types::{Type, TypeGroup},
};
use errors::TypeKind;
use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Untyped;

impl Type for Untyped {
    fn knd(&self) -> TypeKind {
        TypeKind::Untyped
    }
}

impl fmt::Display for Untyped {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("")
    }
}

impl SubstType<Untyped> for Untyped {
    type Target = Self;
    fn subst_type(self, _: &TypeVar, _: &Self) -> Self::Target {
        self
    }
}

impl TypeGroup for Untyped {}
