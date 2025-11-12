use super::{Exceptions, types::Type};
use check::Kindcheck;
use derivations::Derivation;
use errors::{NoKinding, check_error::CheckError};
use grammar::DerivationRule;
use std::collections::HashSet;
use syntax::env::Environment;

impl Kindcheck for Type {
    type Lang = Exceptions;
    fn check_kind(&self, _: Environment<Self::Lang>) -> Result<Derivation<Self::Lang>, CheckError> {
        Err(NoKinding::new("Exceptions").into())
    }

    fn rules() -> HashSet<DerivationRule> {
        HashSet::new()
    }
}

#[cfg(test)]
mod check_tests {
    use super::super::terms::term_tests::{example_term1, example_term2};
    use check::Typecheck;
    use syntax::types::Unit;

    #[test]
    fn check1() {
        let result = example_term1().check(Default::default()).unwrap();
        let expected = Unit::new().into();
        assert_eq!(result.ret_ty(), expected)
    }

    #[test]
    fn check2() {
        let result = example_term2().check(Default::default()).unwrap();
        let expected = Unit::new().into();
        assert_eq!(result.ret_ty(), expected)
    }
}
