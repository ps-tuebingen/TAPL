use crate::{Rule, RuleDescribe, Symbol};
use syntax::{language::Language, values::Loc};

impl<Lang> RuleDescribe for Loc<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(Symbol::Location, "Location")
    }
}
