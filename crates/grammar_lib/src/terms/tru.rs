use crate::{Rule, RuleDescribe, symbols::Keyword};
use syntax::terms::{Term, True};

impl<T> RuleDescribe for True<T>
where
    T: Term,
{
    fn rule() -> Rule {
        Rule::new(Keyword::True.into(), "True")
    }
}
