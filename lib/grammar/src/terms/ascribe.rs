use crate::{GrammarRuleDescribe, Rule, Symbol, symbols::SpecialChar};
use syntax::{language::Language, terms::Ascribe};

impl<Lang> GrammarRuleDescribe for Ascribe<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(
            vec![Symbol::Term, SpecialChar::Colon.into(), Symbol::Type].into(),
            "Ascription",
        )
    }
}
