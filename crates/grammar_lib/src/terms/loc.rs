use crate::{Rule, RuleDescribe, Symbol};
use syntax::terms::{Loc, Term};

impl<T> RuleDescribe for Loc<T>
where
    T: Term,
{
    fn rule() -> Rule {
        Rule::new(Symbol::Location, "Location")
    }
}
