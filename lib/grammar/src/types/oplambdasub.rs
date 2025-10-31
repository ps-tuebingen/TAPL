use crate::{GrammarRule, GrammarRuleDescribe, Symbol, symbols::SpecialChar};
use syntax::{language::Language, types::OpLambdaSub};

impl<Lang> GrammarRuleDescribe for OpLambdaSub<Lang>
where
    Lang: Language,
{
    fn rule() -> GrammarRule {
        GrammarRule::new(
            vec![
                SpecialChar::Lambda.into(),
                Symbol::Type,
                SpecialChar::Dot.into(),
                Symbol::Type,
            ]
            .into(),
            "Operator Abstraction",
        )
    }
}
