use crate::{Rule, RuleDescribe, Symbol};
use syntax::terms::{Something, Term};

impl<T> RuleDescribe for Something<T>
where
    T: Term,
{
    fn rule() -> Rule {
        Rule::new(
            Symbol::ctor("something", None, vec![Symbol::Term]),
            "Something",
        )
    }
}
