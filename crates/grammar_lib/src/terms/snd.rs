use crate::{Rule, RuleDescribe, Symbol, symbols::Keyword};
use syntax::terms::{Snd, Term};

impl<T> RuleDescribe for Snd<T>
where
    T: Term,
{
    fn rule() -> Rule {
        Rule::new(Symbol::dot(Keyword::Snd.into()), "Second Projection")
    }
}
