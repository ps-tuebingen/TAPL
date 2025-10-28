use derivations::Derivation;
use errors::check_error::CheckError;
use grammar::DerivationRule;
use std::collections::HashSet;
use syntax::{env::Environment, language::Language};

pub trait Kindcheck {
    type Lang: Language;

    fn check_kind(
        &self,
        env: Environment<Self::Lang>,
    ) -> Result<Derivation<Self::Lang>, CheckError>;

    fn rules() -> HashSet<DerivationRule>;
}
