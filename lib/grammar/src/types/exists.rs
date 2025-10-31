use crate::{GrammarRule, GrammarRuleDescribe, Symbol, symbols::SpecialChar};
use syntax::{language::Language, types::Exists};

impl<Lang> GrammarRuleDescribe for Exists<Lang>
where
    Lang: Language,
{
    fn rule() -> GrammarRule {
        GrammarRule::new(
            vec![
                SpecialChar::Exists.into(),
                Symbol::double_colon_sep(Symbol::Type, Symbol::Kind),
                SpecialChar::Dot.into(),
                Symbol::Type,
            ]
            .into(),
            "Existential Type",
        )
    }
}
