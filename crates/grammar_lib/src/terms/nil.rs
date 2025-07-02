use crate::{Rule, RuleDescribe, Symbol};
use syntax::{
    terms::{Nil, Term},
    types::Type,
};

impl<T, Ty> RuleDescribe for Nil<T, Ty>
where
    T: Term,
    Ty: Type,
{
    fn rule() -> Rule {
        Rule::new(
            Symbol::ctor("nil", Some(Symbol::Term), vec![Symbol::Term]),
            "Empty List",
        )
    }
}
