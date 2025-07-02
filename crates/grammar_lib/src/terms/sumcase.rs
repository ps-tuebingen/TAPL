use crate::{Rule, RuleDescribe, Symbol};
use syntax::terms::{SumCase, Term};

impl<T> RuleDescribe for SumCase<T>
where
    T: Term,
{
    fn rule() -> Rule {
        Rule::new(
            Symbol::case(vec![
                Symbol::pt(Symbol::ctor_pt("inl", 1), Symbol::Term),
                Symbol::pt(Symbol::ctor_pt("inr", 1), Symbol::Term),
            ]),
            "Sum Case",
        )
    }
}
