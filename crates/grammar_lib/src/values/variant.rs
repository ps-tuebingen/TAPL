use crate::{Rule, RuleDescribe, Symbol};
use syntax::{
    types::Type,
    values::{Value, Variant},
};

impl<V, Ty> RuleDescribe for Variant<V, Ty>
where
    V: Value,
    Ty: Type,
{
    fn rule() -> Rule {
        Rule::new(Symbol::variant(Symbol::Value), "Variant")
    }
}
