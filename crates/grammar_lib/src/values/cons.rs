use crate::{Rule, RuleDescribe, Symbol};
use syntax::{types::Type, values::Cons, values::Value};

impl<V, Ty> RuleDescribe for Cons<V, Ty>
where
    V: Value,
    Ty: Type,
{
    fn rule() -> Rule {
        Rule::new(Symbol::call_annot("cons", 2, Symbol::Value), "Cons")
    }
}
