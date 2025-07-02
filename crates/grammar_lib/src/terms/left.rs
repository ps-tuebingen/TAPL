use crate::{Rule, RuleDescribe, Symbol};
use syntax::{
    terms::{Left, Term},
    types::Type,
};

impl<T, Ty> RuleDescribe for Left<T, Ty>
where
    T: Term,
    Ty: Type,
{
    fn rule() -> Rule {
        Rule::new(
            Symbol::ctor("inl", Some(Symbol::Type), vec![Symbol::Term]),
            "Left Injection",
        )
    }
}
