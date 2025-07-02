use crate::{Rule, RuleDescribe, Symbol, symbols::Keyword};
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
            Symbol::ctor(Keyword::Right, Some(Symbol::Type), vec![Symbol::Value]),
            "Right Injection",
        )
    }
}
