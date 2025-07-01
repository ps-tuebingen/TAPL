use crate::{Rule, RuleDescribe, Symbol};
use syntax::terms::{ListCase, Term};

impl<T> RuleDescribe for ListCase<T>
where
    T: Term,
{
    fn rule() -> Rule {
        Rule::new(
            Symbol::case(vec![Symbol::pt("nil", 0), Symbol::pt("cons", 2)]),
            "Cast",
        )
    }
}
