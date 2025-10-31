use crate::{GrammarRule, GrammarRuleDescribe, Symbol, symbols::SpecialChar};
use syntax::{language::Language, types::OpLambda};

impl<Lang> GrammarRuleDescribe for OpLambda<Lang>
where
    Lang: Language,
{
    fn rule() -> GrammarRule {
        GrammarRule::new(
            vec![
                SpecialChar::Lambda.into(),
                Symbol::Kind,
                SpecialChar::Dot.into(),
                Symbol::Type,
            ]
            .into(),
            "Operator Abstraction",
        )
    }
}
