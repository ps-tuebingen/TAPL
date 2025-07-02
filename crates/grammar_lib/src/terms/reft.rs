use crate::{Rule, RuleDescribe, Symbol};
use syntax::terms::{Ref, Term};

impl<T> RuleDescribe for Ref<T>
where
    T: Term,
{
    fn rule() -> Rule {
        Rule::new(Symbol::ctor("ref", None, vec![Symbol::Term]), "Reference")
    }
}
