use crate::{GrammarRule, GrammarRuleDescribe, Symbol};
use syntax::{language::Language, terms::Variable};

impl<Lang> GrammarRuleDescribe for Variable<Lang>
where
    Lang: Language,
{
    fn rule() -> GrammarRule {
        GrammarRule::new(Symbol::Variable, "Variable")
    }
}
