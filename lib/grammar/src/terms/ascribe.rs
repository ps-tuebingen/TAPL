use crate::{GrammarRule, GrammarRuleDescribe, Symbol, symbols::SpecialChar};
use syntax::{language::Language, terms::Ascribe};

impl<Lang> GrammarRuleDescribe for Ascribe<Lang>
where
    Lang: Language,
{
    fn rule() -> GrammarRule {
        GrammarRule::new(
            vec![Symbol::Term, SpecialChar::Colon.into(), Symbol::Type].into(),
            "Ascription",
        )
    }
}
