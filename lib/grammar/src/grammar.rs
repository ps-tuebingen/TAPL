use crate::{rules::GrammarRule, symbols::Symbol};
use std::fmt;

/// Describes the grammar of terms/types/etc
/// In the form `t::= r1 | r2 | ...`
#[derive(Debug)]
pub struct Grammar {
    /// The symbol for this defintion (e.g. `t` for terms)
    pub symbol: Symbol,
    /// A description for the definition (e.g. "Term" or "Type")
    pub description: String,
    /// A list of alternatives defining the grammar
    pub alternatives: Vec<GrammarRule>,
}

impl Grammar {
    /// create a term grammar with given alternatives
    pub fn term(alternatives: Vec<GrammarRule>) -> Grammar {
        Grammar {
            symbol: Symbol::Term,
            description: "Term".to_owned(),
            alternatives,
        }
    }

    /// create a type grammar with given alternatives
    pub fn ty(alternatives: Vec<GrammarRule>) -> Grammar {
        Grammar {
            symbol: Symbol::Type,
            description: "Type".to_owned(),
            alternatives,
        }
    }

    /// Crate a value grammar with given alternatives
    pub fn value(alternatives: Vec<GrammarRule>) -> Grammar {
        Grammar {
            symbol: Symbol::Value,
            description: "Value".to_owned(),
            alternatives,
        }
    }
}

impl fmt::Display for Grammar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}::=\t\t\t{}", self.symbol, self.description)?;
        for alt in self.alternatives.iter() {
            writeln!(f, "| {alt}",)?;
        }
        Ok(())
    }
}
