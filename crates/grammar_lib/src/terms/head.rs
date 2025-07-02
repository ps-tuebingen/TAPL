use crate::{Rule, RuleDescribe, Symbol};
use syntax::{
    terms::{Head, Term},
    types::Type,
};

impl<T, Ty> RuleDescribe for Head<T, Ty>
where
    T: Term,
    Ty: Type,
{
    fn rule() -> Rule {
        Rule::new(
            Symbol::ctor("head", Some(Symbol::Type), vec![Symbol::Term]),
            "List Head",
        )
    }
}
