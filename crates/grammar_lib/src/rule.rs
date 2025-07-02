use crate::symbols::Symbol;
use std::fmt;

#[derive(Debug)]
pub struct Rule {
    symbol: Symbol,
    description: String,
}

impl Rule {
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
