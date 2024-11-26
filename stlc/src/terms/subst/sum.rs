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
        .subst("x".to_owned(), "y".to_owned().into());
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
        .subst("x".to_owned(), "y".to_owned().into());
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
        .subst("x".to_owned(), "y".to_owned().into());
        let expected = SumCase {
            bound_term: Box::new("y".to_owned().into()),
            left_var: "x".to_owned(),
            left_term: Box::new("x".to_owned().into()),
            right_var: "y".to_owned().into(),
            right_term: Box::new("y".to_owned().into()),
        };
        assert_eq!(result, expected)
    }
}
