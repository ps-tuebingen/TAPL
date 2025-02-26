use crate::types::{Type, TypeVar};
use std::collections::VecDeque;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Constraint {
    pub left: Type,
    pub right: Type,
}

impl Constraint {
    pub fn subst(self, v: &TypeVar, ty: Type) -> Constraint {
        Constraint {
            left: self.left.subst(v, ty.clone()),
            right: self.right.subst(v, ty),
        }
    }
}

pub fn subst_constrs(constrs: &mut VecDeque<Constraint>, v: &TypeVar, ty: Type) {
    let old_constrs = constrs.clone();
    constrs.clear();
    for constr in old_constrs.into_iter() {
        constrs.push_back(constr.subst(v, ty.clone()));
    }
}
