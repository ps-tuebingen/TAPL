use super::Subst;
use crate::{
    syntax::{Nothing, SomeCase, Something, Term},
    traits::free_vars::{fresh_var, FreeVars},
};
use common::Var;

impl Subst for Nothing {
    type Target = Nothing;
    fn subst(self, _: &Var, _: Term) -> Self::Target {
        self
    }
}
impl Subst for Something {
    type Target = Something;
    fn subst(self, var: &Var, term: Term) -> Self::Target {
        Something {
            term: self.term.subst(var, term),
        }
    }
}
impl Subst for SomeCase {
    type Target = SomeCase;
    fn subst(self, var: &Var, term: Term) -> Self::Target {
        let mut free_v = self.free_vars();
        free_v.insert(self.some_var.clone());
        let fresh_var = fresh_var(&free_v);
        let some_subst = self
            .some_rhs
            .subst(&self.some_var, fresh_var.clone().into());
        SomeCase {
            bound_term: self.bound_term.subst(var, term.clone()),
            none_rhs: self.none_rhs.subst(var, term.clone()),
            some_var: fresh_var,
            some_rhs: some_subst.subst(var, term),
        }
    }
}

#[cfg(test)]
mod opt_tests {
    use super::{Nothing, Something, Subst};
    use crate::types::Type;

    #[test]
    fn subst_nothing() {
        let result = Nothing {
            inner_type: Type::Bool,
        }
        .subst(&"x".to_owned(), "y".to_owned().into());
        let expected = Nothing {
            inner_type: Type::Bool,
        };
        assert_eq!(result, expected)
    }

    #[test]
    fn subst_some() {
        let result = Something {
            term: Box::new("x".to_owned().into()),
        }
        .subst(&"x".to_owned(), "y".to_owned().into());
        let expected = Something {
            term: Box::new("y".to_owned().into()),
        };
        assert_eq!(result, expected)
    }
}
