use crate::{Rule, RuleDescribe, Symbol};
use syntax::{terms::Term, values::Num};

impl<T> RuleDescribe for Num<T>
where
    T: Term,
{
    fn rule() -> Rule {
        Rule::new(Symbol::term("int"), "Number")
    }
}
