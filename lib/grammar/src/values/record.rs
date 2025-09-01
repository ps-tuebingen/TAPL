use crate::{Rule, RuleDescribe, Symbol};
use syntax::{language::Language, values::Record};

impl<Lang> RuleDescribe for Record<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(Symbol::record(Symbol::Value), "Record")
    }
}
