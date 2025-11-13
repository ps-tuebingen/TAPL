use crate::{
    subst::{SubstTerm, SubstType},
    terms::Term,
    types::TypeGroup,
    values::ValueGroup,
};
use std::fmt;

pub struct LanguageFeatures {
    pub typed: bool,
    pub kinded: bool,
    pub subtyped: bool,
    pub normalizing: bool,
    pub evaluating: bool,
}

impl LanguageFeatures {
    #[must_use] pub const fn new() -> Self {
        Self {
            typed: false,
            kinded: false,
            subtyped: false,
            normalizing: false,
            evaluating: false,
        }
    }

    #[must_use] pub const fn with_eval(mut self) -> Self {
        self.evaluating = true;
        self
    }

    #[must_use] pub const fn with_typed(mut self) -> Self {
        self.typed = true;
        self
    }

    #[must_use] pub const fn with_subtyped(mut self) -> Self {
        self.subtyped = true;
        self
    }

    #[must_use] pub const fn with_kinded(mut self) -> Self {
        self.kinded = true;
        self
    }

    #[must_use] pub const fn with_normalizing(mut self) -> Self {
        self.normalizing = true;
        self
    }
}

pub trait Language: fmt::Display + fmt::Debug + Clone + PartialEq {
    type Term: Term
        + SubstTerm<Lang = Self, Target = Self::Term>
        + SubstType<Lang = Self, Target = Self::Term>;
    type Type: TypeGroup + SubstType<Lang = Self, Target = Self::Type>;
    type Value: ValueGroup<Lang = Self> + Into<Self::Term>;

    fn describe(&self) -> &str;

    fn features() -> LanguageFeatures;
}

impl Default for LanguageFeatures {
    fn default() -> Self {
        Self::new()
    }
}
