use crate::rule::Rule;

pub struct Grammar {
    symbol: String,
    description: String,
    alternatives: Vec<Rule>,
}

impl Grammar {
    pub fn term(alternatives: Vec<Rule>) -> Grammar {
        Grammar {
            symbol: "t".to_owned(),
            description: "Term".to_owned(),
            alternatives,
        }
    }

    pub fn ty(alternatives: Vec<Rule>) -> Grammar {
        Grammar {
            symbol: "T".to_owned(),
            description: "Type".to_owned(),
            alternatives,
        }
    }

    pub fn value(alternatives: Vec<Rule>) -> Grammar {
        Grammar {
            symbol: "v".to_owned(),
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
