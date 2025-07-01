use crate::{Rule, RuleDescribe, Symbol};
use syntax::terms::{Term, Tuple};

impl<T> RuleDescribe for Tuple<T>
where
    T: Term,
{
    fn rule() -> Rule {
        Rule::new(Symbol::Tuple, "Tuple")
    }
}
