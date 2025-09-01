use crate::{Rule, RuleDescribe, Symbol};
use syntax::{language::Language, types::Tuple};

impl<Lang> RuleDescribe for Tuple<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(Symbol::tuple(Symbol::Type), "Tuple Type")
    }
}
