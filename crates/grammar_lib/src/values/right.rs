use crate::{Rule, RuleDescribe, Symbol};
use syntax::{
    types::Type,
    values::{Right, Value},
};

impl<V, Ty> RuleDescribe for Right<V, Ty>
where
    V: Value,
    Ty: Type,
{
    fn rule() -> Rule {
        Rule::new(
            Symbol::ctor("inr", Some(Symbol::Type), vec![Symbol::Value]),
            "Right Injection",
        )
    }
}
