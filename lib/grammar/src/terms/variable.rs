use crate::{GrammarRuleDescribe, Rule, Symbol};
use syntax::{language::Language, terms::Variable};

impl<Lang> GrammarRuleDescribe for Variable<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(Symbol::Variable, "Variable")
    }
}
