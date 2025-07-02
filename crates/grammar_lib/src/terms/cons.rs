use crate::{Rule, RuleDescribe, Symbol};
use syntax::{
    terms::{Cons, Term},
    types::Type,
};

impl<T, Ty> RuleDescribe for Cons<T, Ty>
where
    T: Term,
    Ty: Type,
{
    fn rule() -> Rule {
        Rule::new(
            Symbol::ctor("Cons", Some(Symbol::Type), vec![Symbol::Term, Symbol::Term]),
            "Cons",
        )
    }
}
