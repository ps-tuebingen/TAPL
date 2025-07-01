use crate::{Rule, RuleDescribe, Symbol};
use syntax::terms::{Term, Unit};

impl<T> RuleDescribe for Unit<T>
where
    T: Term,
{
    fn rule() -> Rule {
        Rule::new(Symbol::term("unit"), "Unit")
    }
}
