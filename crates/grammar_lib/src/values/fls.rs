use crate::{Rule, RuleDescribe, Symbol};
use syntax::{terms::Term, values::False};

impl<T> RuleDescribe for False<T>
where
    T: Term,
{
    fn rule() -> Rule {
        Rule::new(Symbol::term("false"), "False")
    }
}
