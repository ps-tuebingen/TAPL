use crate::{Rule, RuleDescribe, Symbol};
use syntax::{language::Language, terms::TryWithVal};

impl<Lang> RuleDescribe for TryWithVal<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(Symbol::try_catch(), "Try-Catch")
    }
}
