use crate::{Rule, RuleDescribe, Symbol};
use syntax::terms::{Term, Try};

impl<T> RuleDescribe for Try<T>
where
    T: Term,
{
    fn rule() -> Rule {
        Rule::new(Symbol::tryt(), "Variant Case")
    }
}
