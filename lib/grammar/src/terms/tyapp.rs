use crate::{GrammarRule, GrammarRuleDescribe, Symbol, symbols::SpecialChar};
use syntax::{language::Language, terms::TyApp};

impl<Lang> GrammarRuleDescribe for TyApp<Lang>
where
    Lang: Language,
{
    fn rule() -> GrammarRule {
        GrammarRule::new(
            vec![Symbol::Term, SpecialChar::Space.into(), Symbol::Type].into(),
            "Type Application",
        )
    }
}
