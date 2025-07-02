use crate::{Rule, RuleDescribe, Symbol};
use syntax::{
    terms::{Exception, Term},
    types::Type,
};

impl<T, Ty> RuleDescribe for Exception<T, Ty>
where
    T: Term,
    Ty: Type,
{
    fn rule() -> Rule {
        Rule::new(
            Symbol::ctor("error", Some(Symbol::Type), vec![Symbol::Term]),
            "Exception",
        )
    }
}
