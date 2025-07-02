use crate::{Rule, RuleDescribe, Symbol};
use syntax::terms::{Fix, Term};

impl<T> RuleDescribe for Fix<T>
where
    T: Term,
{
    fn rule() -> Rule {
        Rule::new(Symbol::ctor("fix", None, vec![Symbol::Term]), "Fixed Point")
    }
}
