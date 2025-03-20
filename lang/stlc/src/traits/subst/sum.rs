use super::Subst;
use crate::{
    syntax::{Left, Right, SumCase, Term},
    traits::free_vars::{fresh_var, FreeVars},
    Var,
};

impl Subst for Left {
    type Target = Left;
    fn subst(self, var: &Var, term: Term) -> Self::Target {
        Left {
            left_term: self.left_term.subst(var, term),
            ty: self.ty,
        }
    }
}

impl Subst for Right {
    type Target = Right;
    fn subst(self, var: &Var, term: Term) -> Self::Target {
        Right {
            right_term: self.right_term.subst(var, term),
            ty: self.ty,
        }
    }
}

impl Subst for SumCase {
    type Target = SumCase;
    fn subst(self, var: &Var, term: Term) -> Self::Target {
        let mut free_v = self.free_vars();
        free_v.insert(self.left_var.clone());
        free_v.insert(self.right_var.clone());
        free_v.insert(var.clone());

        let new_left = fresh_var(&free_v);
        free_v.insert(new_left.clone());
        let new_right = fresh_var(&free_v);

        let left_subst = self
            .left_term
            .subst(&self.left_var, new_left.clone().into());
        let right_subst = self
            .right_term
            .subst(&self.right_var, new_right.clone().into());

        SumCase {
            bound_term: self.bound_term.subst(var, term.clone()),
            left_var: new_left,
            left_term: left_subst.subst(var, term.clone()),
            right_var: new_right,
            right_term: right_subst.subst(var, term),
        }
    }
}

#[cfg(test)]
mod sum_tests {
    use super::{Left, Right, Subst, SumCase};
    use crate::types::Type;

    #[test]
    fn subst_left() {
        let result = Left {
            left_term: Box::new("x".to_owned().into()),
            right_ty: Type::Bool,
        }
        .subst(&"x".to_owned(), "y".to_owned().into());
        let expected = Left {
            left_term: Box::new("y".to_owned().into()),
            right_ty: Type::Bool,
        };
        assert_eq!(result, expected)
    }

    #[test]
    fn subst_right() {
        let result = Right {
            right_term: Box::new("x".to_owned().into()),
            left_ty: Type::Bool,
        }
        .subst(&"x".to_owned(), "y".to_owned().into());
        let expected = Right {
            right_term: Box::new("y".to_owned().into()),
            left_ty: Type::Bool,
        };
        assert_eq!(result, expected)
    }

    #[test]
    fn subst_sumcase() {
        let result = SumCase {
            bound_term: Box::new("x".to_owned().into()),
            left_var: "x".to_owned(),
            left_term: Box::new("x".to_owned().into()),
            right_var: "y".to_owned(),
            right_term: Box::new("x".to_owned().into()),
        }
        .subst(&&"x".to_owned(), "y".to_owned().into());
        let expected = SumCase {
            bound_term: Box::new("y".to_owned().into()),
            left_var: "x0".to_owned(),
            left_term: Box::new("x0".to_owned().into()),
            right_var: "x1".to_owned().into(),
            right_term: Box::new("y".to_owned().into()),
        };
        assert_eq!(result, expected)
    }
}
