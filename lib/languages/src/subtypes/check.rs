use super::{Subtypes, types::Type};
use check::{Kindcheck, Normalize};
use derivations::{Derivation, NormalizingDerivation};
use errors::{NoKinding, check_error::CheckError};
use grammar::DerivationRule;
use std::collections::HashSet;
use syntax::env::Environment;

impl Kindcheck for Type {
    type Lang = Subtypes;
    fn check_kind(&self, _: Environment<Self::Lang>) -> Result<Derivation<Self::Lang>, CheckError> {
        Err(NoKinding::new("Subtypes").into())
    }

    fn rules() -> HashSet<DerivationRule> {
        HashSet::new()
    }
}

impl Normalize for Type {
    type Lang = Subtypes;
    fn normalize(self, _: Environment<Self::Lang>) -> Derivation<Self::Lang> {
        NormalizingDerivation::empty(self).into()
    }

    fn rules() -> HashSet<DerivationRule> {
        HashSet::new()
    }
}
