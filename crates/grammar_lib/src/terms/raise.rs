use crate::{Rule, RuleDescribe, Symbol};
use syntax::{
    terms::{Raise, Term},
    types::Type,
};

impl<T, Ty> RuleDescribe for Raise<T, Ty>
where
    T: Term,
    Ty: Type,
{
    fn rule() -> Rule {
        Rule::new(
            Symbol::ctor("raise", Some(Symbol::Term), vec![Symbol::Term]),
            "Raise",
        )
    }
}
