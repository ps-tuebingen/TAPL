use crate::{Rule, RuleDescribe, symbols::Keyword};
use syntax::terms::{False, Term};

impl<T> RuleDescribe for False<T>
where
    T: Term,
{
    fn rule() -> Rule {
        Rule::new(Keyword::False.into(), "False")
    }
}
