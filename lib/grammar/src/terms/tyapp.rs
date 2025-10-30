use crate::{GrammarRuleDescribe, Rule, Symbol, symbols::SpecialChar};
use syntax::{language::Language, terms::TyApp};

impl<Lang> GrammarRuleDescribe for TyApp<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(
            vec![Symbol::Term, SpecialChar::Space.into(), Symbol::Type].into(),
            "Type Application",
        )
    }
}
