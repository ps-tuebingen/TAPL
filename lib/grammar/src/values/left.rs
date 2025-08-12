use crate::{Rule, RuleDescribe, Symbol, symbols::Keyword};
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
            Symbol::ctor(Keyword::Left, Some(Symbol::Type), vec![Symbol::Value]),
            "Left Injection",
        )
    }
}
