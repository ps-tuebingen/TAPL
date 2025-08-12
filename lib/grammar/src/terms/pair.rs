use crate::{Rule, RuleDescribe, Symbol};
use syntax::terms::{Pair, Term};

impl<T> RuleDescribe for Pair<T>
where
    T: Term,
{
    fn rule() -> Rule {
        Rule::new(Symbol::pair(Symbol::Term), "Pair")
    }
}
