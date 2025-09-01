use crate::{Rule, RuleDescribe, Symbol};
use syntax::{language::Language, terms::If};

impl<Lang> RuleDescribe for If<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(Symbol::ift(), "If")
    }
}
