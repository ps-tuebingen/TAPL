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
            Symbol::call_annot("inr", 1, Symbol::Value),
            "Right Injection",
        )
    }
}
