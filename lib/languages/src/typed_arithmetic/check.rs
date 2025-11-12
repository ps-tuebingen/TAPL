use super::{TypedArithmetic, types::Type};
use check::Kindcheck;
use derivations::Derivation;
use errors::{NoKinding, check_error::CheckError};
use grammar::DerivationRule;
use std::collections::HashSet;
use syntax::env::Environment;

impl Kindcheck for Type {
    type Lang = TypedArithmetic;
    fn check_kind(&self, _: Environment<Self::Lang>) -> Result<Derivation<Self::Lang>, CheckError> {
        Err(NoKinding::new("Typed Arithmetic").into())
    }

    fn rules() -> HashSet<DerivationRule> {
        HashSet::new()
    }
}
