use super::Subst;
use crate::{
    syntax::{Let, Term},
    Var,
};

impl Subst for Let {
    type Target = Let;
    fn subst(self, var: Var, term: Term) -> Self::Target {
        let in_term = if self.var == var {
            self.in_term
        } else {
            self.in_term.subst(var.clone(), term.clone())
        };
        Let {
            var: self.var,
            bound_term: self.bound_term.subst(var, term),
            in_term,
        }
    }
}

#[cfg(test)]
mod let_tests {
    use super::{Let, Subst};

    #[test]
    fn subst_let() {
        let result = Let {
            var: "y".to_owned(),
            in_term: Box::new("y".to_owned().into()),
            bound_term: Box::new("x".to_owned().into()),
        }
        .subst("x".to_owned(), "y".to_owned().into());
        let expected = Let {
            var: "y".to_owned(),
            bound_term: Box::new("y".to_owned().into()),
            in_term: Box::new("y".to_owned().into()),
        };
        assert_eq!(result, expected)
    }
}
