use crate::{GrammarRuleDescribe, Rule, Symbol};
use syntax::{language::Language, terms::App};

impl<Lang> GrammarRuleDescribe for App<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(Symbol::app(Symbol::Term, Symbol::Term), "Application")
    }
}
