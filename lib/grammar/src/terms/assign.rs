use crate::{Rule, RuleDescribe, Symbol};
use syntax::terms::{Assign, Term};

impl<T> RuleDescribe for Assign<T>
where
    T: Term,
{
    fn rule() -> Rule {
        Rule::new(Symbol::assign(), "Assignment")
    }
}
