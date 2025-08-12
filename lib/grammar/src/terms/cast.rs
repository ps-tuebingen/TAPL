use crate::{Rule, RuleDescribe, Symbol};
use syntax::{
    terms::{Cast, Term},
    types::Type,
};

impl<T, Ty> RuleDescribe for Cast<T, Ty>
where
    T: Term,
    Ty: Type,
{
    fn rule() -> Rule {
        Rule::new(Symbol::cast(), "Cast")
    }
}
