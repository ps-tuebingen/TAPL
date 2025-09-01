use crate::{Rule, RuleDescribe, Symbol};
use syntax::{language::Language, terms::App};

impl<Lang> RuleDescribe for App<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(Symbol::app(Symbol::Term, Symbol::Term), "Application")
    }
}
