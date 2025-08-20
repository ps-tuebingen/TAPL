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
pub struct UntypedArithmetic;

impl Language for UntypedArithmetic {
    type Term = Term;
    type Type = Untyped<Term>;
    type Value = Value;

    fn describe(&self) -> &str {
        "Untyped Arithmetic Expressions"
    }
}

impl LanguageDescribe for UntypedArithmetic {
    fn grammars() -> LanguageGrammar {
        LanguageGrammar {
            term_grammar: Term::grammar(),
            type_grammar: Untyped::<Term>::grammar(),
            value_grammar: Value::grammar(),
            include_kinds: false,
        }
    }
}

#[cfg(test)]
mod term_tests {
    use super::terms::Term;
    use eval::Eval;
    use syntax::{
        terms::{If, IsZero, Num, Pred, Succ},
        values::Num as NumVal,
    };

    #[test]
    fn eval_simple() {
        let term: Term = Succ::new(Succ::new(Pred::new(Num::new(0)))).into();
        let result = term.eval_start().unwrap();
        let expected = NumVal::new(1).into();
        assert_eq!(result.val(), expected)
    }

    #[test]
    fn eval_complex() {
        let term: Term = If::new(
            IsZero::new(Succ::new(Num::new(0))),
            Pred::new(Succ::new(Num::new(0))),
            Succ::new(Pred::new(Num::new(0))),
        )
        .into();
        let result = term.eval_start().unwrap();
        let expected = NumVal::new(0).into();
        assert_eq!(result.val(), expected)
    }
}
