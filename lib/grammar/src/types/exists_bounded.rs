use crate::{GrammarRule, GrammarRuleDescribe, Symbol, symbols::SpecialChar};
use syntax::{language::Language, types::ExistsBounded};

impl<Lang> GrammarRuleDescribe for ExistsBounded<Lang>
where
    Lang: Language,
{
    fn rule() -> GrammarRule {
        GrammarRule::new(
            vec![
                SpecialChar::Exists.into(),
                Symbol::less_colon_sep(Symbol::Type, Symbol::Type),
                SpecialChar::Dot.into(),
                Symbol::Type,
            ]
            .into(),
            "Existential Type",
        )
    }
}
