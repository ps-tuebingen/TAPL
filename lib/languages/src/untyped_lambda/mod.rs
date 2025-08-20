pub mod eval;
pub mod parser;
pub mod terms;
pub mod values;

use crate::Language;
use grammar::{GrammarDescribe, LanguageDescribe, LanguageGrammar};
use syntax::untyped::Untyped;
use terms::Term;
use values::Value;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UntypedLambda;

impl Language for UntypedLambda {
    type Term = Term;
    type Type = Untyped<Term>;
    type Value = Value;

    fn describe(&self) -> &str {
        "Untyped Lambda Calculus"
    }
}

impl LanguageDescribe for UntypedLambda {
    fn grammars() -> LanguageGrammar {
        LanguageGrammar {
            term_grammar: Term::grammar(),
            type_grammar: Untyped::<Term>::grammar(),
            value_grammar: Value::grammar(),
            include_kinds: false,
        }
    }
}
