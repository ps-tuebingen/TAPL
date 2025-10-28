use crate::{GrammarRuleDescribe, Rule, Symbol};
use syntax::{language::Language, types::Record};

impl<Lang> GrammarRuleDescribe for Record<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(Symbol::record(Symbol::Type), "Record Type")
    }
}
