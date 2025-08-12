use crate::{Rule, RuleDescribe, Symbol};
use syntax::terms::{Deref, Term};

impl<T> RuleDescribe for Deref<T>
where
    T: Term,
{
    fn rule() -> Rule {
        Rule::new(Symbol::dereft(), "Dereference")
    }
}
