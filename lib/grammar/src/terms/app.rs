use crate::{GrammarRule, GrammarRuleDescribe, Symbol, symbols::SpecialChar};
use syntax::{language::Language, terms::App};

impl<Lang> GrammarRuleDescribe for App<Lang>
where
    Lang: Language,
{
    fn rule() -> GrammarRule {
        GrammarRule::new(
            vec![Symbol::Term, SpecialChar::Space.into(), Symbol::Term].into(),
            "Application",
        )
    }
}
