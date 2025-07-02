use crate::{Rule, RuleDescribe, Symbol};
use syntax::values::{Something, Value};

impl<V> RuleDescribe for Something<V>
where
    V: Value,
{
    fn rule() -> Rule {
        Rule::new(
            Symbol::ctor("something", None, vec![Symbol::Value]),
            "Something",
        )
    }
}
