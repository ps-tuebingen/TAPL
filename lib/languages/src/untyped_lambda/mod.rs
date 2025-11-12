pub mod parser;
pub mod terms;
pub mod values;

use crate::Language;
use grammar::{GrammarDescribe, LanguageDescribe, LanguageGrammar, LanguageRules};
use std::fmt;
use syntax::{language::LanguageFeatures, untyped::Untyped};
use terms::Term;
use values::Value;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UntypedLambda;

impl Language for UntypedLambda {
    type Term = Term;
    type Type = Untyped<UntypedLambda>;
    type Value = Value;

    fn describe(&self) -> &str {
        "Untyped Lambda Calculus"
    }

    fn features() -> LanguageFeatures {
        LanguageFeatures::new().with_eval()
    }
}

impl LanguageDescribe for UntypedLambda {
    fn rules() -> LanguageRules {
        LanguageRules {
            typing: <Term as ::check::Typecheck>::rules(),
            subtyping: <Untyped<UntypedLambda> as ::check::Subtypecheck>::rules(),
            kinding: <Untyped<UntypedLambda> as ::check::Kindcheck>::rules(),
            normalizing: <Untyped<UntypedLambda> as ::check::Normalize>::rules(),
            eval: <Term as ::eval::Eval>::rules(),
        }
    }
    fn grammars() -> LanguageGrammar {
        LanguageGrammar {
            term_grammar: Term::grammar(),
            type_grammar: Untyped::<UntypedLambda>::grammar(),
            value_grammar: Value::grammar(),
            include_kinds: false,
        }
    }
}

impl fmt::Display for UntypedLambda {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("untyped-lambda")
    }
}
