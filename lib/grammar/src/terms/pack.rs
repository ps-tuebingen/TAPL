use crate::{GrammarRuleDescribe, Rule, Symbol};
use syntax::{language::Language, terms::Pack};

impl<Lang> GrammarRuleDescribe for Pack<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(Symbol::pack(Symbol::Term), "Package")
    }
}
