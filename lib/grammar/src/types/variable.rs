use crate::{Rule, RuleDescribe, Symbol};
use syntax::{language::Language, types::TypeVariable};

impl<Lang> RuleDescribe for TypeVariable<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(Symbol::Variable, "Type Variable")
    }
}
