use crate::{Rule, RuleDescribe, Symbol, symbols::Keyword};
use syntax::terms::{ListCase, Term};

impl<T> RuleDescribe for ListCase<T>
where
    T: Term,
{
    fn rule() -> Rule {
        Rule::new(
            Symbol::case(vec![
                Symbol::pt(Symbol::ctor_pt(Keyword::Nil, 0), Symbol::Term),
                Symbol::pt(Symbol::ctor_pt(Keyword::Cons, 2), Symbol::Term),
            ]),
            "Cast",
        )
    }
}
