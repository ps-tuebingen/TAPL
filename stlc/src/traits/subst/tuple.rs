use super::Subst;
use crate::{
    syntax::{Proj, Term, Tup},
    Var,
};

impl Subst for Tup {
    type Target = Tup;
    fn subst(self, var: Var, term: Term) -> Self::Target {
        Tup {
            terms: self
                .terms
                .into_iter()
                .map(|t| t.subst(var.clone(), term.clone()))
                .collect(),
        }
    }
}

impl Subst for Proj {
    type Target = Proj;
    fn subst(self, var: Var, term: Term) -> Self::Target {
        Proj {
            tup: self.tup.subst(var, term),
            ind: self.ind,
        }
    }
}

#[cfg(test)]
mod tuple_test {
    use super::{Proj, Subst, Tup};

    #[test]
    fn subst_tup() {
        let result = Tup {
            terms: vec!["x".to_owned().into(), "y".to_owned().into()],
        }
        .subst("x".to_owned(), "y".to_owned().into());
        let expected = Tup {
            terms: vec!["y".to_owned().into(), "y".to_owned().into()],
        };
        assert_eq!(result, expected)
    }

    #[test]
    fn subst_proj() {
        let result = Proj {
            tup: Box::new("x".to_owned().into()),
            ind: 0,
        }
        .subst("x".to_owned(), "y".to_owned().into());
        let expected = Proj {
            tup: Box::new("y".to_owned().into()),
            ind: 0,
        };
        assert_eq!(result, expected)
    }
}
