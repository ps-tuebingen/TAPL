use crate::{GrammarRuleDescribe, Rule, Symbol};
use syntax::{language::Language, terms::Deref};

impl<Lang> GrammarRuleDescribe for Deref<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(Symbol::dereft(), "Dereference")
    }
}
