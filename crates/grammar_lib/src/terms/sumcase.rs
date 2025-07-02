use crate::{Rule, RuleDescribe, Symbol, symbols::Keyword};
use syntax::terms::{SumCase, Term};

impl<T> RuleDescribe for SumCase<T>
where
    T: Term,
{
    fn rule() -> Rule {
        Rule::new(
            Symbol::case(vec![
                Symbol::pt(Symbol::ctor_pt(Keyword::Left, 1), Symbol::Term),
                Symbol::pt(Symbol::ctor_pt(Keyword::Right, 1), Symbol::Term),
            ]),
            "Sum Case",
        )
    }
}
