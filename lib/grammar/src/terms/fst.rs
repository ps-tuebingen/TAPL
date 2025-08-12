use crate::{Rule, RuleDescribe, Symbol, symbols::Keyword};
use syntax::terms::{Fst, Term};

impl<T> RuleDescribe for Fst<T>
where
    T: Term,
{
    fn rule() -> Rule {
        Rule::new(Symbol::dot(Keyword::Fst.into()), "First Projection")
    }
}
