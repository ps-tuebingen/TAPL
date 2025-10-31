use crate::{GrammarRule, GrammarRuleDescribe, Symbol, symbols::SpecialChar};
use syntax::{language::Language, terms::Lambda};

impl<Lang> GrammarRuleDescribe for Lambda<Lang>
where
    Lang: Language,
{
    fn rule() -> GrammarRule {
        GrammarRule::new(
            vec![
                SpecialChar::Lambda.into(),
                Symbol::colon_sep(Symbol::Variable, Symbol::Type),
                SpecialChar::Dot.into(),
                Symbol::Term,
            ]
            .into(),
            "Lambda Abstracion",
        )
    }
}
