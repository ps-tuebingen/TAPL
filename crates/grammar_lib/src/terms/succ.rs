use crate::{Rule, RuleDescribe, Symbol};
use syntax::terms::{Succ, Term};

impl<T> RuleDescribe for Succ<T>
where
    T: Term,
{
    fn rule() -> Rule {
        Rule::new(Symbol::call("succ", 1, Symbol::Term), "Succ")
    }
}
