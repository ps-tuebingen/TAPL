use crate::{Rule, RuleDescribe, Symbol};
use syntax::terms::{Projection, Term};

impl<T> RuleDescribe for Projection<T>
where
    T: Term,
{
    fn rule() -> Rule {
        Rule::new(Symbol::dot(Symbol::Number), "Projection")
    }
}
