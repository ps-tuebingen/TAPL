use crate::symbols::Symbol;

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
