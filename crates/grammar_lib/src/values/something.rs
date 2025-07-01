use crate::{Rule, RuleDescribe, Symbol};
use syntax::values::{Something, Value};

impl<V> RuleDescribe for Something<V>
where
    V: Value,
{
    fn rule() -> Rule {
        Rule::new(Symbol::call("something", 1, Symbol::Value), "Something")
    }
}
