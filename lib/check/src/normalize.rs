use derivations::Derivation;
use grammar::DerivationRule;
use std::collections::HashSet;
use std::rc::Rc;
use syntax::{env::Environment, language::Language};

/// Trait for normalizing Types
pub trait Normalize {
    /// the language types belong to
    type Lang: Language;
    /// Normalize a given term in a given envrionment
    fn normalize(self, env: Environment<Self::Lang>) -> Derivation<Self::Lang>;
    /// Rules used to normalize `Self`
    /// for most types this is empty
    fn rules() -> HashSet<DerivationRule>;
}

impl<T> Normalize for Rc<T>
where
    T: Normalize + Clone,
{
    type Lang = T::Lang;
    fn normalize(self, env: Environment<Self::Lang>) -> Derivation<Self::Lang> {
        Self::unwrap_or_clone(self).normalize(env)
    }

    fn rules() -> HashSet<DerivationRule> {
        T::rules()
    }
}
