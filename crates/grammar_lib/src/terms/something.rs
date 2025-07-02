use crate::{Rule, RuleDescribe, Symbol, symbols::Keyword};
use syntax::terms::{Something, Term};

impl<T> RuleDescribe for Something<T>
where
    T: Term,
{
    fn rule() -> Rule {
        Rule::new(
            Symbol::ctor(Keyword::Something, None, vec![Symbol::Term]),
            "Something",
        )
    }
}
