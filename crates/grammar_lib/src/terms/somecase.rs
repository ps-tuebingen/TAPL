use crate::{Rule, RuleDescribe, Symbol, symbols::Keyword};
use syntax::terms::{SomeCase, Term};

impl<T> RuleDescribe for SomeCase<T>
where
    T: Term,
{
    fn rule() -> Rule {
        Rule::new(
            Symbol::case(vec![
                Symbol::pt(Symbol::ctor_pt(Keyword::Nothing, 0), Symbol::Term),
                Symbol::pt(Symbol::ctor_pt(Keyword::Something, 1), Symbol::Term),
            ]),
            "Option Case",
        )
    }
}
