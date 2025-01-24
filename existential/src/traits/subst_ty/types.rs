use super::SubstTy;
use crate::types::{Type, TypeVar};

impl SubstTy for Type {
    fn subst_ty(self, v: &TypeVar, ty: Type) -> Type {
        match self {
            Type::Var(var) => {
                if var == *v {
                    ty
                } else {
                    Type::Var(var)
                }
            }
            Type::Unit => Type::Unit,
            Type::Nat => Type::Nat,
            Type::Bool => Type::Bool,
            Type::Fun { from, to } => Type::Fun {
                from: Box::new((*from).subst_ty(v, ty.clone())),
                to: Box::new((*to).subst_ty(v, ty.clone())),
            },
            Type::Package { ty_var, ty: inner } => {
                if ty_var == *v {
                    Type::Package { ty_var, ty: inner }
                } else {
                    Type::Package {
                        ty_var,
                        ty: Box::new(inner.subst_ty(v, ty)),
                    }
                }
            }
            Type::Record(recs) => Type::Record(
                recs.into_iter()
                    .map(|(label, term)| (label, term.subst_ty(v, ty.clone())))
                    .collect(),
            ),
        }
    }
}
