use super::SubstTy;
use crate::types::{Type, TypeVar};
use std::collections::HashMap;

impl SubstTy for Type {
    fn subst_ty(self, v: &TypeVar, ty: Type) -> Self {
        match self {
            Type::Var(var) => {
                if var == *v {
                    ty
                } else {
                    Type::Var(var)
                }
            }
            Type::Top => Type::Top,
            Type::Nat => Type::Nat,
            Type::Fun { from, to } => Type::Fun {
                from: Box::new(from.subst_ty(v, ty.clone())),
                to: Box::new(to.subst_ty(v, ty)),
            },
            Type::Forall {
                var,
                sup_ty,
                ty: typ,
            } => {
                let sup_subst = Box::new(sup_ty.subst_ty(v, ty.clone()));

                if var == *v {
                    Type::Forall {
                        var,
                        sup_ty: sup_subst,
                        ty: typ,
                    }
                } else {
                    Type::Forall {
                        var,
                        sup_ty: sup_subst,
                        ty: Box::new(typ.subst_ty(v, ty)),
                    }
                }
            }
            Type::Exists {
                var,
                sup_ty,
                ty: inner,
            } => {
                let sup_subst = sup_ty.subst_ty(v, ty.clone());
                if var == *v {
                    Type::Exists {
                        var,
                        sup_ty: Box::new(sup_subst),
                        ty: inner,
                    }
                } else {
                    Type::Exists {
                        var,
                        sup_ty: Box::new(sup_subst),
                        ty: Box::new(inner.subst_ty(v, ty)),
                    }
                }
            }
            Type::Record(recs) => {
                let mut new_recs = HashMap::new();
                for (lb, typ) in recs.into_iter() {
                    let ty_subst = typ.subst_ty(v, ty.clone());
                    new_recs.insert(lb, ty_subst);
                }
                Type::Record(new_recs)
            }
        }
    }
}
