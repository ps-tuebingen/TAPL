use crate::{Rule, RuleDescribe, Symbol};
use syntax::terms::{Snd, Term};

impl<T> RuleDescribe for Snd<T>
where
    T: Term,
{
    fn rule() -> Rule {
        Rule::new(Symbol::dot(Symbol::term("snd")), "Second Projection")
    }
}
