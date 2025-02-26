use super::Subst;
use crate::{
    syntax::{IsZero, Pred, Succ, Term, Zero},
    Var,
};

impl Subst for Zero {
    type Target = Zero;
    fn subst(self, _: &Var, _: Term) -> Self::Target {
        self
    }
}

impl Subst for Succ {
    type Target = Succ;
    fn subst(self, var: &Var, term: Term) -> Self::Target {
        Succ {
            term: self.term.subst(var, term),
        }
    }
}

impl Subst for Pred {
    type Target = Pred;
    fn subst(self, var: &Var, term: Term) -> Self::Target {
        Pred {
            term: self.term.subst(var, term),
        }
    }
}

impl Subst for IsZero {
    type Target = IsZero;
    fn subst(self, var: &Var, term: Term) -> Self::Target {
        IsZero {
            term: self.term.subst(var, term),
        }
    }
}

#[cfg(test)]
mod nat_tests {
    use super::{IsZero, Pred, Subst, Succ, Zero};

    #[test]
    fn subst_zero() {
        let result = Zero.subst(&"x".to_owned(), "y".to_owned().into());
        let expected = Zero;
        assert_eq!(result, expected)
    }

    #[test]
    fn subst_succ() {
        let result = Succ {
            term: Box::new("x".to_owned().into()),
        }
        .subst(&"x".to_owned(), "y".to_owned().into());
        let expected = Succ {
            term: Box::new("y".to_owned().into()),
        };
        assert_eq!(result, expected)
    }

    #[test]
    fn subst_pred() {
        let result = Pred {
            term: Box::new("x".to_owned().into()),
        }
        .subst(&"x".to_owned(), "y".to_owned().into());
        let expected = Pred {
            term: Box::new("y".to_owned().into()),
        };
        assert_eq!(result, expected)
    }

    #[test]
    fn subst_iszero() {
        let result = IsZero {
            term: Box::new("x".to_owned().into()),
        }
        .subst(&"x".to_owned(), "y".to_owned().into());
        let expected = IsZero {
            term: Box::new("y".to_owned().into()),
        };
        assert_eq!(result, expected)
    }
}
