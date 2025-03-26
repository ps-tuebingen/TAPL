use crate::types::{Type, TypeVar};

pub mod lambda;
pub mod lambda_sub;
pub mod nat;
pub mod pack;
pub mod term;
pub mod types;

pub trait SubstTy {
    fn subst_ty(self, v: &TypeVar, ty: Type) -> Self;
}
