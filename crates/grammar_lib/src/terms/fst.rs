use crate::{Rule, RuleDescribe, Symbol};
use syntax::terms::{Fst, Term};

impl<T> RuleDescribe for Fst<T>
where
    T: Term,
{
    fn rule() -> Rule {
        Rule::new(Symbol::dot(Symbol::term("fst")), "First Projection")
    }
}
