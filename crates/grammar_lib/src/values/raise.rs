use crate::{Rule, RuleDescribe, Symbol};
use syntax::{
    types::Type,
    values::{Raise, Value},
};

impl<V, Ty> RuleDescribe for Raise<V, Ty>
where
    V: Value,
    Ty: Type,
{
    fn rule() -> Rule {
        Rule::new(
            Symbol::ctor("raise", Some(Symbol::Type), vec![Symbol::Value]),
            "Raise",
        )
    }
}
