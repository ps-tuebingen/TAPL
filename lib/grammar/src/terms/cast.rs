use crate::{Rule, RuleDescribe, Symbol};
use syntax::{language::Language, terms::Cast};

impl<Lang> RuleDescribe for Cast<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(Symbol::cast(), "Cast")
    }
}
