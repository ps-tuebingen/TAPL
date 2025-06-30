use crate::{Rule, RuleDescribe, Symbol};
use syntax::terms::{If, Term};

impl<T> RuleDescribe for If<T>
where
    T: Term,
{
    fn rule() -> Rule {
        Rule::new(Symbol::If, "If")
    }
}
