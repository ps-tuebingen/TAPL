use derivations::Derivation;
use errors::check_error::CheckError;
use grammar::DerivationRule;
use std::collections::HashSet;
use syntax::{env::Environment, language::Language};

/// Trait for type checking terms in a language
pub trait Typecheck {
    /// The language this term belongs to
    type Lang: Language;

    /// check with empty environment
    /// # Errors
    /// Returns an error if input is not well-typed
    fn check_start(&self) -> Result<Derivation<Self::Lang>, CheckError> {
        self.check(Environment::default())
    }

    /// check a term with given environment
    /// # Errors
    /// returns an error if input is not well-typed
    fn check(&self, env: Environment<Self::Lang>) -> Result<Derivation<Self::Lang>, CheckError>;

    /// Rules used to check the current term
    /// for singular terms this is usually a single rule
    /// but for language term enums this contains all rules for possible terms
    fn rules() -> HashSet<DerivationRule>;
}
