use crate::{GrammarRuleDescribe, Rule, Symbol};
use syntax::{language::Language, values::Pack};

impl<Lang> GrammarRuleDescribe for Pack<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(Symbol::pack(Symbol::Value), "Package")
    }
}
