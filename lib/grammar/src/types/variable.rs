use crate::{GrammarRule, GrammarRuleDescribe, Symbol};
use syntax::{language::Language, types::TypeVariable};

impl<Lang> GrammarRuleDescribe for TypeVariable<Lang>
where
    Lang: Language,
{
    fn rule() -> GrammarRule {
        GrammarRule::new(Symbol::Variable, "Type Variable")
    }
}
