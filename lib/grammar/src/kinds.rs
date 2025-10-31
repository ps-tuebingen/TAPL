use crate::{Grammar, GrammarDescribe, GrammarRule, Symbol, symbols::SpecialChar};
use syntax::kinds::Kind;

impl GrammarDescribe for Kind {
    fn grammar() -> Grammar {
        Grammar {
            symbol: Symbol::Kind,
            description: "Kind".to_owned(),
            alternatives: vec![
                GrammarRule::new(SpecialChar::Star.into(), "Star Kind"),
                GrammarRule::new(
                    vec![Symbol::Kind, SpecialChar::DoubleArrow.into(), Symbol::Kind].into(),
                    "Arrow Kind",
                ),
            ],
        }
    }
}
