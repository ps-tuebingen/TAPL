use crate::{GrammarRule, GrammarRuleDescribe, Symbol, symbols::SpecialChar};
use syntax::{language::Language, types::Forall};

impl<Lang> GrammarRuleDescribe for Forall<Lang>
where
    Lang: Language,
{
    fn rule() -> GrammarRule {
        GrammarRule::new(
            vec![
                SpecialChar::Forall.into(),
                Symbol::double_colon_sep(Symbol::Typevariable, Symbol::Kind),
                SpecialChar::Dot.into(),
                Symbol::Type,
            ]
            .into(),
            "Universal Type",
        )
    }
}
