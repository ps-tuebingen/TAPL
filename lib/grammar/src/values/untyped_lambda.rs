use crate::{GrammarRule, GrammarRuleDescribe, Symbol, symbols::SpecialChar};
use syntax::{language::Language, values::UntypedLambda};

impl<Lang> GrammarRuleDescribe for UntypedLambda<Lang>
where
    Lang: Language,
{
    fn rule() -> GrammarRule {
        GrammarRule::new(
            vec![
                SpecialChar::Lambda.into(),
                Symbol::Variable,
                SpecialChar::Dot.into(),
                Symbol::Value,
            ]
            .into(),
            "Lambda Abstraction",
        )
    }
}
