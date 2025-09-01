use crate::{Rule, RuleDescribe, Symbol};
use syntax::{language::Language, values::Tuple};

impl<Lang> RuleDescribe for Tuple<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(Symbol::tuple(Symbol::Value), "Tuple")
    }
}
