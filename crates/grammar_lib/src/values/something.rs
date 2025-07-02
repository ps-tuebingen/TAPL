use crate::{Rule, RuleDescribe, Symbol, symbols::Keyword};
use syntax::values::{Something, Value};

impl<V> RuleDescribe for Something<V>
where
    V: Value,
{
    fn rule() -> Rule {
        Rule::new(
            Symbol::ctor(Keyword::Something, None, vec![Symbol::Value]),
            "Something",
        )
    }
}
