use crate::{Rule, RuleDescribe, Symbol};
use syntax::{language::Language, terms::Pair};

impl<Lang> RuleDescribe for Pair<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(Symbol::pair(Symbol::Term), "Pair")
    }
}
