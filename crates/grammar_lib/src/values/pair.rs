use crate::{Rule, RuleDescribe, Symbol};
use syntax::values::{Pair, Value};

impl<V> RuleDescribe for Pair<V>
where
    V: Value,
{
    fn rule() -> Rule {
        Rule::new(Symbol::pair(Symbol::Value), "Pair")
    }
}
