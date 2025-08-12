use crate::{Rule, RuleDescribe, Symbol, symbols::Keyword};
use syntax::terms::{Succ, Term};

impl<T> RuleDescribe for Succ<T>
where
    T: Term,
{
    fn rule() -> Rule {
        Rule::new(
            Symbol::ctor(Keyword::Succ, None, vec![Symbol::Term]),
            "Succ",
        )
    }
}
