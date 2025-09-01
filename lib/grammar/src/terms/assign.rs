use crate::{Rule, RuleDescribe, Symbol};
use syntax::{language::Language, terms::Assign};

impl<Lang> RuleDescribe for Assign<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(Symbol::assign(), "Assignment")
    }
}
