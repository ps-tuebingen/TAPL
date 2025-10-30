pub mod check;
pub mod eval;
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
pub struct Subtypes;

impl Language for Subtypes {
    type Term = Term;
    type Type = Type;
    type Value = Value;

    fn describe(&self) -> &str {
        "STLC with Subtyping"
    }

    fn features() -> LanguageFeatures {
        LanguageFeatures::new()
            .with_eval()
            .with_typed()
            .with_subtyped()
    }
}

impl LanguageDescribe for Subtypes {
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
            include_kinds: false,
        }
    }
}

impl fmt::Display for Subtypes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("subtypes")
    }
}
