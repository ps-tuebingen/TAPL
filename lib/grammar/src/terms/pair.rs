use crate::{GrammarRuleDescribe, Rule, Symbol};
use syntax::{language::Language, terms::Pair};

impl<Lang> GrammarRuleDescribe for Pair<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(Symbol::pair(Symbol::Term), "Pair")
    }
}
