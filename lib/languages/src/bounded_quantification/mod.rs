pub mod check;
pub mod eval;
pub mod parser;
pub mod terms;
pub mod types;
pub mod values;

use crate::Language;
use check::Typecheck;
use grammar::{GrammarDescribe, LanguageDescribe, LanguageGrammar, LanguageRules};
use std::fmt;
use syntax::language::LanguageFeatures;

use terms::Term;
use types::Type;
use values::Value;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BoundedQuantification;

impl Language for BoundedQuantification {
    type Term = Term;
    type Type = Type;
    type Value = Value;

    fn describe(&self) -> &str {
        "System F with Bounded Quantification"
    }

    fn features() -> LanguageFeatures {
        LanguageFeatures::new()
            .with_eval()
            .with_typed()
            .with_subtyped()
    }
}

impl LanguageDescribe for BoundedQuantification {
    fn grammars() -> LanguageGrammar {
        LanguageGrammar {
            term_grammar: Term::grammar(),
            type_grammar: Type::grammar(),
            value_grammar: Value::grammar(),
            include_kinds: true,
        }
    }

    fn rules() -> LanguageRules {
        LanguageRules {
            typing: <Term as Typecheck>::rules(),
            subtyping: <Term as Subtypecheck>::rules(),
            kinding: <Term as Kindcheck>::rules(),
            normalizing: <Term as Normalize>::rules(),
            eval: <Term as Eval>::rules(),
        }
    }
}

impl fmt::Display for BoundedQuantification {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("bounded-quantification")
    }
}
