use crate::symbols::Symbol;
use std::fmt;

/// A rule used in [`crate::Grammar`]
#[derive(Debug)]
pub struct GrammarRule {
    /// The symbol defining the rule
    pub symbol: Symbol,
    /// A description for the rule
    pub description: String,
}

impl GrammarRule {
    /// Create a rule from a given symbol and description
    #[must_use] pub fn new(symbol: Symbol, desc: &str) -> Self {
        Self {
            symbol,
            description: desc.to_owned(),
        }
    }
}

impl fmt::Display for GrammarRule {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}\t\t\t{}", self.symbol, self.description)
    }
}
