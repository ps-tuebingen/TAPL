use crate::{Rule, RuleDescribe, Symbol};
use syntax::{language::Language, terms::Variable};

impl<Lang> RuleDescribe for Variable<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(Symbol::Variable, "Variable")
    }
}
