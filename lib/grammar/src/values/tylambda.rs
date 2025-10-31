use crate::{GrammarRule, GrammarRuleDescribe, Symbol, symbols::SpecialChar};
use syntax::{language::Language, values::TyLambda};

impl<Lang> GrammarRuleDescribe for TyLambda<Lang>
where
    Lang: Language,
{
    fn rule() -> GrammarRule {
        GrammarRule::new(
            vec![
                SpecialChar::Lambda.into(),
                Symbol::double_colon_sep(Symbol::Variable, Symbol::Kind),
                SpecialChar::Dot.into(),
                Symbol::Value,
            ]
            .into(),
            "Type Abstraction",
        )
    }
}
