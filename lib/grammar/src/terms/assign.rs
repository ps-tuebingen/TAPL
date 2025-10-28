use crate::{GrammarRuleDescribe, Rule, Symbol};
use syntax::{language::Language, terms::Assign};

impl<Lang> GrammarRuleDescribe for Assign<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(Symbol::assign(), "Assignment")
    }
}
