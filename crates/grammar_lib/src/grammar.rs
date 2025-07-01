use crate::{rule::Rule, symbol::Symbol};

pub struct Grammar {
    symbol: Symbol,
    description: String,
    alternatives: Vec<Rule>,
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

pub struct LanguageGrammar {
    pub term_grammar: Grammar,
    pub type_grammar: Grammar,
    pub value_grammar: Grammar,
}
