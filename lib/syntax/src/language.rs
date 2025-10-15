use crate::{
    subst::{SubstTerm, SubstType},
    terms::Term,
    types::TypeGroup,
    values::ValueGroup,
};
use std::fmt;

pub struct LanguageFeatures {
    typed: bool,
    kinded: bool,
    subtyped: bool,
    normalizing: bool,
    evaluating: bool,
}

impl LanguageFeatures {
    pub fn new() -> LanguageFeatures {
        LanguageFeatures {
            typed: false,
            kinded: false,
            subtyped: false,
            normalizing: false,
            evaluating: false,
        }
    }

    pub fn with_eval(mut self) -> Self {
        self.evaluating = true;
        self
    }

    pub fn with_typed(mut self) -> Self {
        self.typed = true;
        self
    }

    pub fn with_subtyped(mut self) -> Self {
        self.subtyped = true;
        self
    }

    pub fn with_kinded(mut self) -> Self {
        self.kinded = true;
        self
    }

    pub fn with_normalizing(mut self) -> Self {
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
    fn default() -> LanguageFeatures {
        LanguageFeatures::new()
    }
}
