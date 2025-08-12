use crate::{Rule, RuleDescribe, Symbol};
use syntax::terms::{Term, TryWithVal};

impl<T> RuleDescribe for TryWithVal<T>
where
    T: Term,
{
    fn rule() -> Rule {
        Rule::new(Symbol::try_catch(), "Try-Catch")
    }
}
