use crate::{Rule, RuleDescribe, Symbol};
use syntax::{language::Language, types::Mu};

impl<Lang> RuleDescribe for Mu<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(Symbol::mu(), "Mu Type")
    }
}
