use crate::{Rule, RuleDescribe, Symbol};
use syntax::{language::Language, terms::Loc};

impl<Lang> RuleDescribe for Loc<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(Symbol::Location, "Location")
    }
}
