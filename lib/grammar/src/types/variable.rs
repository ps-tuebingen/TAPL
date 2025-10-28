use crate::{GrammarRuleDescribe, Rule, Symbol};
use syntax::{language::Language, types::TypeVariable};

impl<Lang> GrammarRuleDescribe for TypeVariable<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(Symbol::Variable, "Type Variable")
    }
}
