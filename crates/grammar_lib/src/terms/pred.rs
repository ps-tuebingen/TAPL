use crate::{Rule, RuleDescribe, Symbol, symbols::Keyword};
use syntax::terms::{Pred, Term};

impl<T> RuleDescribe for Pred<T>
where
    T: Term,
{
    fn rule() -> Rule {
        Rule::new(
            Symbol::ctor(Keyword::Pred, None, vec![Symbol::Term]),
            "Pred",
        )
    }
}
