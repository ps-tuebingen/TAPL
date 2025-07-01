use crate::{Rule, RuleDescribe, Symbol};
use syntax::{
    terms::{Term, Unpack},
    types::Type,
};

impl<T, Ty> RuleDescribe for Unpack<T, Ty>
where
    T: Term,
    Ty: Type,
{
    fn rule() -> Rule {
        Rule::new(Symbol::Unpack, "Unpack")
    }
}
