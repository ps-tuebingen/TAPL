use crate::symbols::Symbol;
use std::fmt;

/// A rule used in [`crate::Grammar`]
#[derive(Debug)]
pub struct Rule {
    /// The symbol defining the rule
    pub symbol: Symbol,
    /// A description for the rule
    pub description: String,
}

impl Rule {
    /// Create a rule from a given symbol and description
    pub fn new(symbol: Symbol, desc: &str) -> Rule {
        Rule {
            symbol,
            description: desc.to_owned(),
        }
    }
}

impl fmt::Display for Rule {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}\t\t\t{}", self.symbol, self.description)
    }
}
