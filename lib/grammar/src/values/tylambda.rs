use crate::{GrammarRuleDescribe, Rule, Symbol, symbols::SpecialChar};
use syntax::{language::Language, values::TyLambda};

impl<Lang> GrammarRuleDescribe for TyLambda<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(
            vec![
                SpecialChar::Lambda.into(),
                Symbol::Variable,
                SpecialChar::DoubleColon.into(),
                Symbol::Kind,
                SpecialChar::Dot.into(),
                Symbol::Value,
            ]
            .into(),
            "Type Abstraction",
        )
    }
}
