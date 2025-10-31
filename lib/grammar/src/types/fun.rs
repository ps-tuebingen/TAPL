use crate::{GrammarRuleDescribe, Rule, Symbol};
use syntax::{language::Language, types::Fun};

impl<Lang> GrammarRuleDescribe for Fun<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(Symbol::arrow(Symbol::Type, Symbol::Type), "Function Type")
    }
}
