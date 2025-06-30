use crate::{Rule, RuleDescribe, Symbol};
use syntax::terms::{Num, Term};

impl<T> RuleDescribe for Num<T>
where
    T: Term,
{
    fn rule() -> Rule {
        Rule::new(Symbol::term("int"), "Number")
    }
}
