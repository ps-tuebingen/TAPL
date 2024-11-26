use crate::syntax::Term;
use crate::Var;

pub mod ascription;
pub mod bool;
pub mod fix;
pub mod lambda;
pub mod let_exp;
pub mod list;
pub mod optional;
pub mod pair;
pub mod record;
pub mod sum;
pub mod term;
pub mod tuple;
pub mod unit;
pub mod variant;

pub trait Subst {
    type Target;
    fn subst(self, var: &Var, term: Term) -> Self::Target;
}

impl Subst for Var {
    type Target = Term;
    fn subst(self, var: &Var, term: Term) -> Self::Target {
        if &self == var {
            term
        } else {
            self.into()
        }
    }
}

impl<T: Subst> Subst for Box<T> {
    type Target = Box<T::Target>;
    fn subst(self, var: &Var, term: Term) -> Self::Target {
        Box::new((*self).subst(var, term))
    }
}

#[cfg(test)]
mod subst_tests {
    use super::Subst;

    #[test]
    fn subst_var() {
        let result = "x".to_owned().subst(&"x".to_owned(), "y".to_owned().into());
        let expected = "y".to_owned().into();
        assert_eq!(result, expected)
    }

    #[test]
    fn no_subst_var() {
        let result = "x".to_owned().subst(&"y".to_owned(), "z".to_owned().into());
        let expected = "x".to_owned().into();
        assert_eq!(result, expected)
    }
}
