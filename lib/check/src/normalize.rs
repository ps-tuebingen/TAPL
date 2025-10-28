use derivations::Derivation;
use grammar::DerivationRule;
use std::collections::HashSet;
use std::rc::Rc;
use syntax::{env::Environment, language::Language};

pub trait Normalize {
    type Lang: Language;
    fn normalize(self, env: Environment<Self::Lang>) -> Derivation<Self::Lang>;
    fn rules() -> HashSet<DerivationRule>;
}

impl<T> Normalize for Rc<T>
where
    T: Normalize + Clone,
{
    type Lang = T::Lang;
    fn normalize(self, env: Environment<Self::Lang>) -> Derivation<Self::Lang> {
        Rc::unwrap_or_clone(self).normalize(env)
    }

    fn rules() -> HashSet<DerivationRule> {
        T::rules()
    }
}
