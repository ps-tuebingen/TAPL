use crate::{Grammar, GrammarDescribe, Rule, Symbol, symbols::SpecialChar};
use syntax::kinds::Kind;

impl GrammarDescribe for Kind {
    fn grammar() -> Grammar {
        Grammar {
            symbol: Symbol::Kind,
            description: "Kind".to_owned(),
            alternatives: vec![
                Rule::new(SpecialChar::Star.into(), "Star Kind"),
                Rule::new(
                    vec![Symbol::Kind, SpecialChar::DoubleArrow.into(), Symbol::Kind].into(),
                    "Arrow Kind",
                ),
            ],
        }
    }
}
