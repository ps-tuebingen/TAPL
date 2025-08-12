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
                    Symbol::Separated {
                        fst: Box::new(Symbol::Kind),
                        separator: Box::new(SpecialChar::DoubleArrow.into()),
                        snd: Box::new(Symbol::Kind),
                    },
                    "Arrow Kind",
                ),
            ],
        }
    }
}
