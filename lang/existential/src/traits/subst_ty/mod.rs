use crate::types::{Type, TypeVar};

pub mod bool;
pub mod fix;
pub mod lambda;
pub mod nat;
pub mod pack;
pub mod record;
pub mod term;
pub mod types;

pub trait SubstTy: Sized {
    fn subst_ty(self, v: &TypeVar, ty: Type) -> Self;
}
