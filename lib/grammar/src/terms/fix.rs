use crate::{Rule, RuleDescribe, Symbol, symbols::Keyword};
use syntax::terms::{Fix, Term};

impl<T> RuleDescribe for Fix<T>
where
    T: Term,
{
    fn rule() -> Rule {
        Rule::new(
            Symbol::ctor(Keyword::Fix, None, vec![Symbol::Term]),
            "Fixed Point",
        )
    }
}
