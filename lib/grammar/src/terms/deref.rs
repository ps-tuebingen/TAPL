use crate::{Rule, RuleDescribe, Symbol};
use syntax::{language::Language, terms::Deref};

impl<Lang> RuleDescribe for Deref<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(Symbol::dereft(), "Dereference")
    }
}
