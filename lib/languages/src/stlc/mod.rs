pub mod check;
pub mod eval;
pub mod parser;
pub mod terms;
pub mod types;
pub mod values;

use crate::Language;
use std::fmt;
use grammar::{GrammarDescribe, LanguageDescribe, LanguageGrammar};
use terms::Term;
use types::Type;
use values::Value;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Stlc;

impl Language for Stlc {
    type Term = Term;
    type Type = Type;
    type Value = Value;

    fn describe(&self) -> &str {
        "Simply-Typed Lambda Calculus"
    }
}

impl LanguageDescribe for Stlc {
    fn grammars() -> LanguageGrammar {
        LanguageGrammar {
            term_grammar: Term::grammar(),
            type_grammar: Type::grammar(),
            value_grammar: Value::grammar(),
            include_kinds: false,
        }
    }
}

impl fmt::Display for Stlc{
    fn fmt(&self,f:&mut fmt::Formatter) -> fmt::Result{
        f.write_str("stlc")
    }
}
