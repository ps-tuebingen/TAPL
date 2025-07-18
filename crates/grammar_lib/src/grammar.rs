use crate::{GrammarDescribe, rule::Rule, symbols::Symbol};
use std::fmt;
use syntax::kinds::Kind;

#[derive(Debug)]
pub struct Grammar {
    pub symbol: Symbol,
    pub description: String,
    pub alternatives: Vec<Rule>,
}

impl Grammar {
    pub fn term(alternatives: Vec<Rule>) -> Grammar {
        Grammar {
            symbol: Symbol::Term,
            description: "Term".to_owned(),
            alternatives,
        }
    }

    pub fn ty(alternatives: Vec<Rule>) -> Grammar {
        Grammar {
            symbol: Symbol::Type,
            description: "Type".to_owned(),
            alternatives,
        }
    }

    pub fn value(alternatives: Vec<Rule>) -> Grammar {
        Grammar {
            symbol: Symbol::Value,
            description: "Value".to_owned(),
            alternatives,
        }
    }
}

#[derive(Debug)]
pub struct LanguageGrammar {
    pub term_grammar: Grammar,
    pub type_grammar: Grammar,
    pub value_grammar: Grammar,
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
