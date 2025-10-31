use crate::{GrammarRule, GrammarRuleDescribe, Symbol};
use syntax::{language::Language, types::Fun};

impl<Lang> GrammarRuleDescribe for Fun<Lang>
where
    Lang: Language,
{
    fn rule() -> GrammarRule {
        GrammarRule::new(Symbol::arrow(Symbol::Type, Symbol::Type), "Function Type")
    }
}
