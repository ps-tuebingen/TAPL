use crate::{Rule, RuleDescribe, Symbol};
use syntax::{
    types::Type,
    values::{Left, Value},
};

impl<V, Ty> RuleDescribe for Left<V, Ty>
where
    V: Value,
    Ty: Type,
{
    fn rule() -> Rule {
        Rule::new(
            Symbol::call_annot("inl", 1, Symbol::Value),
            "Left Injection",
        )
    }
}
