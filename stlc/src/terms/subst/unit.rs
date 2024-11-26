use super::Subst;
use crate::{
    terms::syntax::{Term, Unit},
    Var,
};

impl Subst for Unit {
    type Target = Unit;
    fn subst(self, _: Var, _: Term) -> Self::Target {
        self
    }
}

#[cfg(test)]
mod unit_tests {
    use super::{Subst, Unit};
    #[test]
    fn subst_unit() {
        let result = Unit {}.subst("x".to_owned(), "y".to_owned().into());
        let expected = Unit {};
        assert_eq!(result, expected)
    }
}
