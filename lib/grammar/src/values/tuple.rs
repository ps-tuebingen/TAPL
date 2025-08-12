use crate::{Rule, RuleDescribe, Symbol};
use syntax::values::{Tuple, Value};

impl<V> RuleDescribe for Tuple<V>
where
    V: Value,
{
    fn rule() -> Rule {
        Rule::new(Symbol::tuple(Symbol::Value), "Tuple")
    }
}
