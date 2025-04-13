use super::FreeVars;
use crate::syntax::{IsZero, Pred, Succ, Zero};
use common::Var;
use std::collections::HashSet;

impl FreeVars for Zero {
    fn free_vars(&self) -> HashSet<Var> {
        HashSet::new()
    }
}
impl FreeVars for Succ {
    fn free_vars(&self) -> HashSet<Var> {
        self.term.free_vars()
    }
}
impl FreeVars for Pred {
    fn free_vars(&self) -> HashSet<Var> {
        self.term.free_vars()
    }
}
impl FreeVars for IsZero {
    fn free_vars(&self) -> HashSet<Var> {
        self.term.free_vars()
    }
}

#[cfg(test)]
mod nat_tests {
    use super::{FreeVars, IsZero, Pred, Succ, Zero};
    use std::collections::HashSet;

    #[test]
    fn free_vars_zero() {
        let result = Zero.free_vars();
        let expected = HashSet::new();
        assert_eq!(result, expected)
    }

    #[test]
    fn free_vars_succ() {
        let result = Succ {
            term: Box::new("x".to_owned().into()),
        }
        .free_vars();
        let expected = HashSet::from(["x".to_owned()]);
        assert_eq!(result, expected)
    }

    #[test]
    fn free_vars_pred() {
        let result = Pred {
            term: Box::new("x".to_owned().into()),
        }
        .free_vars();
        let expected = HashSet::from(["x".to_owned()]);
        assert_eq!(result, expected)
    }

    #[test]
    fn free_vars_iszero() {
        let result = IsZero {
            term: Box::new("x".to_owned().into()),
        }
        .free_vars();
        let expected = HashSet::from(["x".to_owned()]);
        assert_eq!(result, expected)
    }
}
