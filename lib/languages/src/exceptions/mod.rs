pub mod check;
pub mod eval;
pub mod parser;
pub mod terms;
pub mod types;
pub mod values;

use crate::Language;
use grammar::{GrammarDescribe, LanguageDescribe, LanguageGrammar};
use std::fmt;
use terms::Term;
use types::Type;
use values::Value;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Exceptions;

impl Language for Exceptions {
    type Term = Term;
    type Type = Type;
    type Value = Value;

    fn describe(&self) -> &str {
        "STLC with Exceptions"
    }
}

impl LanguageDescribe for Exceptions {
    fn grammars() -> LanguageGrammar {
        LanguageGrammar {
            term_grammar: Term::grammar(),
            type_grammar: Type::grammar(),
            value_grammar: Value::grammar(),
            include_kinds: false,
        }
    }
}

impl fmt::Display for Exceptions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("exceptions")
    }
}
