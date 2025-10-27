use crate::{GrammarDescribe, rule::Rule, symbols::Symbol};
use std::fmt;
use syntax::kinds::Kind;

/// Describes the grammar of terms/types/etc
/// In the form `t::= r1 | r2 | ...`
#[derive(Debug)]
pub struct Grammar {
    /// The symbol for this defintion (e.g. `t` for terms)
    pub symbol: Symbol,
    /// A description for the definition (e.g. "Term" or "Type")
    pub description: String,
    /// A list of alternatives defining the grammar
    pub alternatives: Vec<Rule>,
}

impl Grammar {
    /// create a term grammar with given alternatives
    pub fn term(alternatives: Vec<Rule>) -> Grammar {
        Grammar {
            symbol: Symbol::Term,
            description: "Term".to_owned(),
            alternatives,
        }
    }

    /// create a type grammar with given alternatives
    pub fn ty(alternatives: Vec<Rule>) -> Grammar {
        Grammar {
            symbol: Symbol::Type,
            description: "Type".to_owned(),
            alternatives,
        }
    }

    /// Crate a value grammar with given alternatives
    pub fn value(alternatives: Vec<Rule>) -> Grammar {
        Grammar {
            symbol: Symbol::Value,
            description: "Value".to_owned(),
            alternatives,
        }
    }
}

/// Grammar of a language
#[derive(Debug)]
pub struct LanguageGrammar {
    /// The grammar of terms
    pub term_grammar: Grammar,
    /// The grammar of types
    pub type_grammar: Grammar,
    /// The grammar of values
    pub value_grammar: Grammar,
    /// Is the language kinded?
    pub include_kinds: bool,
}

impl fmt::Display for LanguageGrammar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", self.term_grammar)?;
        writeln!(f, "{}", self.type_grammar)?;
        writeln!(f, "{}", self.value_grammar)?;
        if self.include_kinds {
            writeln!(f, "{}", Kind::grammar())?;
        }
        Ok(())
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
