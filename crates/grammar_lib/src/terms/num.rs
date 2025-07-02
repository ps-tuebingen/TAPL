use crate::{Rule, RuleDescribe, symbols::SpecialChar};
use syntax::terms::{Num, Term};

impl<T> RuleDescribe for Num<T>
where
    T: Term,
{
    fn rule() -> Rule {
        Rule::new(SpecialChar::Number.into(), "Number")
    }
}
