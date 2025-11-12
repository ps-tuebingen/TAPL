use super::{Subtypes, types::Type};
use check::Normalize;
use derivations::{Derivation, NormalizingDerivation};
use grammar::DerivationRule;
use std::collections::HashSet;
use syntax::env::Environment;

impl Normalize for Type {
    type Lang = Subtypes;
    fn normalize(self, _: Environment<Self::Lang>) -> Derivation<Self::Lang> {
        NormalizingDerivation::empty(self).into()
    }

    fn rules() -> HashSet<DerivationRule> {
        HashSet::new()
    }
}
