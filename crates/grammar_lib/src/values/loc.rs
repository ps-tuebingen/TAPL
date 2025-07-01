use crate::{Rule, RuleDescribe, Symbol};
use syntax::{terms::Term, values::Loc};

impl<T> RuleDescribe for Loc<T>
where
    T: Term,
{
    fn rule() -> Rule {
        Rule::new(Symbol::Location, "Location")
    }
}
