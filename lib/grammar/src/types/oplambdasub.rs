use crate::{GrammarRuleDescribe, Rule, Symbol, symbols::SpecialChar};
use syntax::{language::Language, types::OpLambdaSub};

impl<Lang> GrammarRuleDescribe for OpLambdaSub<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(
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
