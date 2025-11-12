use super::{SystemF, types::Type};
use check::{Kindcheck, Subtypecheck};
use derivations::Derivation;
use errors::{NoKinding, NoSubtyping, check_error::CheckError};
use grammar::DerivationRule;
use std::collections::HashSet;
use syntax::env::Environment;

impl Subtypecheck for Type {
    type Lang = SystemF;
    fn check_subtype(
        &self,
        _: &Type,
        _: Environment<Self::Lang>,
    ) -> Result<Derivation<Self::Lang>, CheckError> {
        Err(NoSubtyping::new("System F").into())
    }

    fn rules() -> HashSet<DerivationRule> {
        HashSet::new()
    }
}

impl Kindcheck for Type {
    type Lang = SystemF;
    fn check_kind(&self, _: Environment<Self::Lang>) -> Result<Derivation<Self::Lang>, CheckError> {
        Err(NoKinding::new("System F").into())
    }

    fn rules() -> HashSet<DerivationRule> {
        HashSet::new()
    }
}
