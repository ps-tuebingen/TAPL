use crate::{Rule, RuleDescribe, Symbol};
use syntax::terms::{ListCase, Term};

impl<T> RuleDescribe for ListCase<T>
where
    T: Term,
{
    fn rule() -> Rule {
        Rule::new(
            Symbol::case(vec![
                Symbol::pt(Symbol::ctor_pt("nil", 0), Symbol::Term),
                Symbol::pt(Symbol::ctor_pt("cons", 2), Symbol::Term),
            ]),
            "Cast",
        )
    }
}
