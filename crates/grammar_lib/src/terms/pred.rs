use crate::{Rule, RuleDescribe, Symbol};
use syntax::terms::{Pred, Term};

impl<T> RuleDescribe for Pred<T>
where
    T: Term,
{
    fn rule() -> Rule {
        Rule::new(Symbol::ctor("pred", None, vec![Symbol::Term]), "Pred")
    }
}
