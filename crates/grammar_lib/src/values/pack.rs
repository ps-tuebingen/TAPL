use crate::{Rule, RuleDescribe, Symbol};
use syntax::{
    types::Type,
    values::{Pack, Value},
};

impl<V, Ty> RuleDescribe for Pack<V, Ty>
where
    V: Value,
    Ty: Type,
{
    fn rule() -> Rule {
        Rule::new(Symbol::pack(Symbol::Value), "Package")
    }
}
