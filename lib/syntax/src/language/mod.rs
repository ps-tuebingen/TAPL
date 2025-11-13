use crate::{
    subst::{SubstTerm, SubstType},
    terms::Term,
    types::TypeGroup,
    values::ValueGroup,
};
use std::fmt;

mod features;
pub use features::LanguageFeatures;

/// Trait for a language with terms, types and values
pub trait Language: fmt::Display + fmt::Debug + Clone + PartialEq {
    /// Terms of the language
    type Term: Term
        + SubstTerm<Lang = Self, Target = Self::Term>
        + SubstType<Lang = Self, Target = Self::Term>;
    /// Types of the language ([`crate::untyped::Untyped`] for languages with no types)
    type Type: TypeGroup + SubstType<Lang = Self, Target = Self::Type>;
    /// Values of the language
    type Value: ValueGroup<Lang = Self> + Into<Self::Term>;

    /// Describe the language as a string
    fn describe(&self) -> &str;

    fn features() -> LanguageFeatures;
}
