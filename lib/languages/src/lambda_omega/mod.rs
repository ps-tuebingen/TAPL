pub mod check;
pub mod eval;
pub mod parser;
pub mod terms;
pub mod types;
pub mod values;

use crate::Language;
use grammar::{GrammarDescribe, LanguageDescribe, LanguageGrammar};
use std::fmt;
use syntax::language::LanguageFeatures;

use terms::Term;
use types::Type;
use values::Value;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LambdaOmega;

impl Language for LambdaOmega {
    type Term = Term;
    type Type = Type;
    type Value = Value;

    fn describe(&self) -> &str {
        "STLC with Higher Kinded Types"
    }

    fn features() -> LanguageFeatures {
        LanguageFeatures::new()
            .with_eval()
            .with_typed()
            .with_kinded()
    }
}

impl LanguageDescribe for LambdaOmega {
    fn grammars() -> LanguageGrammar {
        LanguageGrammar {
            term_grammar: Term::grammar(),
            type_grammar: Type::grammar(),
            value_grammar: Value::grammar(),
            include_kinds: true,
        }
    }
}

impl fmt::Display for LambdaOmega {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("lambda-omega")
    }
}
