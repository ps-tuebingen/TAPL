use crate::{GrammarRuleDescribe, Rule, Symbol, symbols::SpecialChar};
use syntax::{language::Language, types::OpLambda};

impl<Lang> GrammarRuleDescribe for OpLambda<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(
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
