use crate::{Rule, RuleDescribe, Symbol};
use syntax::{terms::Term, types::Type, values::Nil};

impl<T, Ty> RuleDescribe for Nil<T, Ty>
where
    T: Term,
    Ty: Type,
{
    fn rule() -> Rule {
        Rule::new(
            Symbol::ctor("nil", Some(Symbol::Type), vec![Symbol::Value]),
            "Empty List",
        )
    }
}
