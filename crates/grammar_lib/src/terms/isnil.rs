use crate::{Rule, RuleDescribe, Symbol};
use syntax::{
    terms::{IsNil, Term},
    types::Type,
};

impl<T, Ty> RuleDescribe for IsNil<T, Ty>
where
    T: Term,
    Ty: Type,
{
    fn rule() -> Rule {
        Rule::new(
            Symbol::ctor("isnil", Some(Symbol::Type), vec![Symbol::Term]),
            "Is Nil",
        )
    }
}
