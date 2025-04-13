use super::Subst;
use crate::{
    syntax::{Let, Term},
    traits::free_vars::{fresh_var, FreeVars},
};
use common::Var;

impl Subst for Let {
    type Target = Let;
    fn subst(self, var: &Var, term: Term) -> Self::Target {
        let mut free_v = self.free_vars();
        free_v.insert(self.var.clone());
        free_v.insert(var.clone());

        let new_v = fresh_var(&free_v);
        let in_subst = self.in_term.subst(&self.var, new_v.clone().into());

        Let {
            var: new_v,
            bound_term: self.bound_term.subst(var, term.clone()),
            in_term: in_subst.subst(var, term),
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
        .subst(&"x".to_owned(), "y".to_owned().into());
        let expected = Let {
            var: "x0".to_owned(),
            in_term: Box::new("x0".to_owned().into()),
            bound_term: Box::new("y".to_owned().into()),
        };
        assert_eq!(result, expected)
    }
}
