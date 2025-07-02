pub mod check;
pub mod eval;
pub mod parser;
pub mod terms;
pub mod types;
pub mod values;

use crate::Language;
use grammar::{GrammarDescribe, LanguageDescribe, LanguageGrammar};
use terms::Term;
use types::Type;
use values::Value;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TypedArithmetic;

impl Language for TypedArithmetic {
    type Term = Term;
    type Type = Type;
    type Value = Value;
}
impl LanguageDescribe for TypedArithmetic {
    fn grammars() -> LanguageGrammar {
        LanguageGrammar {
            term_grammar: Term::grammar(),
            type_grammar: Type::grammar(),
            value_grammar: Value::grammar(),
            include_kinds: false,
        }
    }
}
