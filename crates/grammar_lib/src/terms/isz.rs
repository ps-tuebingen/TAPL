use crate::{Rule, RuleDescribe, Symbol, symbols::Keyword};
use syntax::terms::{IsZero, Term};

impl<T> RuleDescribe for IsZero<T>
where
    T: Term,
{
    fn rule() -> Rule {
        Rule::new(
            Symbol::ctor(Keyword::IsZero, None, vec![Symbol::Term]),
            "IsZero",
        )
    }
}
