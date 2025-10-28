use crate::{GrammarRuleDescribe, Rule, Symbol};
use syntax::{language::Language, terms::Tuple};

impl<Lang> GrammarRuleDescribe for Tuple<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(Symbol::tuple(Symbol::Term), "Tuple")
    }
}
