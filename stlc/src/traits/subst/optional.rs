use super::Subst;
use crate::{
    syntax::{Nothing, Something, Term},
    Var,
};

impl Subst for Nothing {
    type Target = Nothing;
    fn subst(self, _: Var, _: Term) -> Self::Target {
        self
    }
}
impl Subst for Something {
    type Target = Something;
    fn subst(self, var: Var, term: Term) -> Self::Target {
        Something {
            term: self.term.subst(var, term),
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
        .subst("x".to_owned(), "y".to_owned().into());
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
        .subst("x".to_owned(), "y".to_owned().into());
        let expected = Something {
            term: Box::new("y".to_owned().into()),
        };
        assert_eq!(result, expected)
    }
}
