use super::Subst;
use crate::{
    terms::syntax::{Pair, Proj1, Proj2, Term},
    Var,
};

impl Subst for Pair {
    type Target = Pair;
    fn subst(self, var: Var, term: Term) -> Self::Target {
        Pair {
            fst: self.fst.subst(var.clone(), term.clone()),
            snd: self.snd.subst(var, term),
        }
    }
}
impl Subst for Proj1 {
    type Target = Proj1;
    fn subst(self, var: Var, term: Term) -> Self::Target {
        Proj1 {
            pair: self.pair.subst(var, term),
        }
    }
}
impl Subst for Proj2 {
    type Target = Proj2;
    fn subst(self, var: Var, term: Term) -> Self::Target {
        Proj2 {
            pair: self.pair.subst(var, term),
        }
    }
}

#[cfg(test)]
mod pair_test {
    use super::{Pair, Proj1, Proj2, Subst};

    #[test]
    fn subst_pair() {
        let result = Pair {
            fst: Box::new("x".to_owned().into()),
            snd: Box::new("y".to_owned().into()),
        }
        .subst("x".to_owned(), "y".to_owned().into());
        let expected = Pair {
            fst: Box::new("y".to_owned().into()),
            snd: Box::new("y".to_owned().into()),
        };
        assert_eq!(result, expected)
    }

    #[test]
    fn subst_proj1() {
        let result = Proj1 {
            pair: Box::new("x".to_owned().into()),
        }
        .subst("x".to_owned(), "y".to_owned().into());
        let expected = Proj1 {
            pair: Box::new("y".to_owned().into()),
        };
        assert_eq!(result, expected)
    }

    #[test]
    fn subst_proj2() {
        let result = Proj2 {
            pair: Box::new("x".to_owned().into()),
        }
        .subst("x".to_owned(), "y".to_owned().into());
        let expected = Proj2 {
            pair: Box::new("y".to_owned().into()),
        };
        assert_eq!(result, expected)
    }
}
