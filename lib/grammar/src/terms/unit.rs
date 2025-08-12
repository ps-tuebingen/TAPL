use crate::{Rule, RuleDescribe, symbols::Keyword};
use syntax::terms::{Term, Unit};

impl<T> RuleDescribe for Unit<T>
where
    T: Term,
{
    fn rule() -> Rule {
        Rule::new(Keyword::Unit.into(), "Unit")
    }
}
