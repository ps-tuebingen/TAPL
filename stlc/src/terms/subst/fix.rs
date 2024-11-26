use super::Subst;
use crate::{
    terms::syntax::{Fix, Term},
    Var,
};

impl Subst for Fix {
    type Target = Fix;
    fn subst(self, var: Var, term: Term) -> Self::Target {
        Fix {
            term: self.term.subst(var, term),
        }
    }
}

#[cfg(test)]
mod fix_tests {
    use super::{Fix, Subst};

    #[test]
    fn subst_fix() {
        let result = Fix {
            term: Box::new("x".to_owned().into()),
        }
        .subst("x".to_owned(), "y".to_owned().into());
        let expected = Fix {
            term: Box::new("y".to_owned().into()),
        };
        assert_eq!(result, expected)
    }
}
