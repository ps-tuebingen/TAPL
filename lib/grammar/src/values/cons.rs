use crate::{Rule, RuleDescribe, Symbol, symbols::Keyword};
use syntax::{types::Type, values::Cons, values::Value};

impl<V, Ty> RuleDescribe for Cons<V, Ty>
where
    V: Value,
    Ty: Type,
{
    fn rule() -> Rule {
        Rule::new(
            Symbol::ctor(
                Keyword::Cons,
                Some(Symbol::Type),
                vec![Symbol::Value, Symbol::Value],
            ),
            "Cons",
        )
    }
}
