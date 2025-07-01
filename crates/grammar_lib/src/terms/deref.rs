use crate::{Rule, RuleDescribe, Symbol};
use syntax::terms::{Deref, Term};

impl<T> RuleDescribe for Deref<T>
where
    T: Term,
{
    fn rule() -> Rule {
        Rule::new(Symbol::call("!", 1, Symbol::Term), "Dereference")
    }
}
