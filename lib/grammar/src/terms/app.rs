use crate::{GrammarRuleDescribe, Rule, Symbol, symbols::SpecialChar};
use syntax::{language::Language, terms::App};

impl<Lang> GrammarRuleDescribe for App<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(
            vec![Symbol::Term, SpecialChar::Space.into(), Symbol::Term].into(),
            "Application",
        )
    }
}
