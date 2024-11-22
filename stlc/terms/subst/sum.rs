use super::Subst;
use crate::{
    terms::syntax::{Left, Right, SumCase, Term},
    Var,
};

impl Subst for Left {
    type Target = Left;
    fn subst(self, var: Var, term: Term) -> Self::Target {
        Left {
            left_term: self.left_term.subst(var, term),
            right_ty: self.right_ty,
        }
    }
}

impl Subst for Right {
    type Target = Right;
    fn subst(self, var: Var, term: Term) -> Self::Target {
        Right {
            right_term: self.right_term.subst(var, term),
            left_ty: self.left_ty,
        }
    }
}

impl Subst for SumCase {
    type Target = SumCase;
    fn subst(self, var: Var, term: Term) -> Self::Target {
        let left_term = if var == self.left_var {
            self.left_term
        } else {
            self.left_term.subst(var.clone(), term.clone())
        };
        let right_term = if var == self.right_var {
            self.right_term
        } else {
            self.right_term.subst(var.clone(), term.clone())
        };
        SumCase {
            bound_term: self.bound_term.subst(var, term),
            left_var: self.left_var,
            left_term,
            right_var: self.right_var,
            right_term,
        }
    }
}
