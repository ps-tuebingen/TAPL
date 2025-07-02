use crate::{Rule, RuleDescribe, Symbol};
use syntax::{
    terms::{Pack, Term},
    types::Type,
};

impl<T, Ty> RuleDescribe for Pack<T, Ty>
where
    T: Term,
    Ty: Type,
{
    fn rule() -> Rule {
        Rule::new(Symbol::pack(Symbol::Term), "Package")
    }
}
