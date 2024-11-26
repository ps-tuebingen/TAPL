use super::Subst;
use crate::{
    syntax::{Ascribe, Term},
    Var,
};

impl Subst for Ascribe {
    type Target = Ascribe;
    fn subst(self, var: &Var, term: Term) -> Self::Target {
        Ascribe {
            ty: self.ty,
            term: self.term.subst(var, term),
        }
    }
}

#[cfg(test)]
mod ascribe_tests {
    use super::{Ascribe, Subst};
    use crate::types::Type;

    #[test]
    fn subst_ascribe() {
        let result = Ascribe {
            term: Box::new("x".to_owned().into()),
            ty: Type::Bool,
        }
        .subst(&"x".to_owned(), "y".to_owned().into());
        let expected = Ascribe {
            term: Box::new("y".to_owned().into()),
            ty: Type::Bool,
        };
        assert_eq!(result, expected)
    }
}
