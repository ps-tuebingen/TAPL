use std::collections::HashSet;

#[derive(PartialEq, Eq, Hash)]
pub enum LanguageFeature {
    Typed,
    Subtyped,
    Kinded,
    Normalizing,
    Evaluating,
}

pub struct LanguageFeatures {
    features: HashSet<LanguageFeature>,
}

impl LanguageFeatures {
    /// Create `Self` with no features enabled
    #[must_use]
    pub fn new() -> Self {
        Self {
            features: HashSet::new(),
        }
    }

    /// Add [`LanguageFeature::Evaluating`] to `Self`
    #[must_use]
    pub fn with_eval(mut self) -> Self {
        self.features.insert(LanguageFeature::Evaluating);
        self
    }

    /// Does `Self` contain [`LanguageFeature::Evaluating`]
    #[must_use]
    pub fn evaluating(&self) -> bool {
        self.features.contains(&LanguageFeature::Evaluating)
    }

    /// Add [`LanguageFeature::Typed`] to `Self`]
    #[must_use]
    pub fn with_typed(mut self) -> Self {
        self.features.insert(LanguageFeature::Typed);
        self
    }

    /// Does `Self` contain [`LanguageFeature::Typed`]
    #[must_use]
    pub fn typed(&self) -> bool {
        self.features.contains(&LanguageFeature::Typed)
    }

    /// Add [`LanguageFeature::Subtyped`] to `Self`
    #[must_use]
    pub fn with_subtyped(mut self) -> Self {
        self.features.insert(LanguageFeature::Subtyped);
        self
    }

    /// Does `Self` contain [`LanguageFeature::Subtyped`]
    #[must_use]
    pub fn subtyped(&self) -> bool {
        self.features.contains(&LanguageFeature::Subtyped)
    }

    /// Add [`LanguageFeature::Kinded`] to `Self`
    #[must_use]
    pub fn with_kinded(mut self) -> Self {
        self.features.insert(LanguageFeature::Kinded);
        self
    }

    /// Does `Self` contain [`LanguageFeature::Kinded`]
    #[must_use]
    pub fn kinded(&self) -> bool {
        self.features.contains(&LanguageFeature::Kinded)
    }

    /// Add [`LanguageFeature::Normalizing`] to `Self`
    #[must_use]
    pub fn with_normalizing(mut self) -> Self {
        self.features.insert(LanguageFeature::Normalizing);
        self
    }

    /// Does `Self` contain [`LanguageFeature::Normalizing`]
    #[must_use]
    pub fn normalizing(&self) -> bool {
        self.features.contains(&LanguageFeature::Normalizing)
    }
}

impl Default for LanguageFeatures {
    fn default() -> Self {
        Self::new()
    }
}
