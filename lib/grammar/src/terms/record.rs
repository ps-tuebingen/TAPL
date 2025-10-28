use crate::{GrammarRuleDescribe, Rule, Symbol};
use syntax::{language::Language, terms::Record};

impl<Lang> GrammarRuleDescribe for Record<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(Symbol::record(Symbol::Term), "Record")
    }
}
