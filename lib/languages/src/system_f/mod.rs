pub mod parser;
pub mod terms;
pub mod types;
pub mod values;

use crate::Language;
use grammar::{GrammarDescribe, LanguageDescribe, LanguageGrammar, LanguageRules};
use std::fmt;
use syntax::language::LanguageFeatures;

use terms::Term;
use types::Type;
use values::Value;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SystemF;

impl Language for SystemF {
    type Term = Term;
    type Type = Type;
    type Value = Value;

    fn describe(&self) -> &'static str {
        "System F"
    }

    fn features() -> LanguageFeatures {
        LanguageFeatures::new().with_eval().with_typed()
    }
}

impl LanguageDescribe for SystemF {
    fn rules() -> LanguageRules {
        LanguageRules {
            typing: <Term as ::check::Typecheck>::rules(),
            subtyping: <Type as ::check::Subtypecheck>::rules(),
            kinding: <Type as ::check::Kindcheck>::rules(),
            normalizing: <Type as ::check::Normalize>::rules(),
            eval: <Term as ::eval::Eval>::rules(),
        }
    }
    fn grammars() -> LanguageGrammar {
        LanguageGrammar {
            term_grammar: Term::grammar(),
            type_grammar: Type::grammar(),
            value_grammar: Value::grammar(),
            include_kinds: true,
        }
    }
}

impl fmt::Display for SystemF {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("system-f")
    }
}
