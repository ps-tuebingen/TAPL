use derivations::Derivation;
use errors::check_error::CheckError;
use grammar::DerivationRule;
use std::collections::HashSet;
use syntax::{env::Environment, language::Language};

/// Check subtyping for types in a language
pub trait Subtypecheck {
    /// The language types belong to
    type Lang: Language;

    /// Check that self is a subtype of sup with a given environment
    /// # Errors
    /// Returns an error if the subtyping relation does not hold
    fn check_subtype(
        &self,
        sup: &<Self::Lang as Language>::Type,
        env: Environment<Self::Lang>,
    ) -> Result<Derivation<Self::Lang>, CheckError>;

    /// Derivation rules for subtyping `Self`
    /// usually one specific rule for `Self` and one for `Self<:Top`
    fn rules() -> HashSet<DerivationRule>;
}
