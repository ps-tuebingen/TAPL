use crate::{Rule, RuleDescribe, symbols::SpecialChar};
use syntax::{terms::Term, values::Num};

impl<T> RuleDescribe for Num<T>
where
    T: Term,
{
    fn rule() -> Rule {
        Rule::new(SpecialChar::Number.into(), "Number")
    }
}
