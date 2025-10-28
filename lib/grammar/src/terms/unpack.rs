use crate::{GrammarRuleDescribe, Rule, Symbol};
use syntax::{language::Language, terms::Unpack};

impl<Lang> GrammarRuleDescribe for Unpack<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(Symbol::unpack(), "Unpack")
    }
}
