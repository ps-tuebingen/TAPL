use derivations::Derivation;
use errors::check_error::CheckError;
use grammar::DerivationRule;
use std::collections::HashSet;
use syntax::{env::Environment, language::Language};

/// Trait for kind checking types
pub trait Kindcheck {
    /// Language the types belong to
    type Lang: Language;

    /// Check the kind of `Self` in a given environment
    /// # Errors
    /// Returns an error if `self` is not well-kinded
    fn check_kind(
        &self,
        env: Environment<Self::Lang>,
    ) -> Result<Derivation<Self::Lang>, CheckError>;

    /// rules used to kind the given type
    fn rules() -> HashSet<DerivationRule>;
}
