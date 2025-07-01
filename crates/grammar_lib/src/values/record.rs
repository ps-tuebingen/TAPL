use crate::{Rule, RuleDescribe, Symbol};
use syntax::values::{Record, Value};

impl<V> RuleDescribe for Record<V>
where
    V: Value,
{
    fn rule() -> Rule {
        Rule::new(Symbol::record(Symbol::Value), "Record")
    }
}
