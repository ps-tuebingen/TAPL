use crate::{Rule, RuleDescribe, Symbol};
use syntax::terms::{SomeCase, Term};

impl<T> RuleDescribe for SomeCase<T>
where
    T: Term,
{
    fn rule() -> Rule {
        Rule::new(
            Symbol::case(vec![Symbol::pt("nothing", 0), Symbol::pt("something", 1)]),
            "Option Case",
        )
    }
}
