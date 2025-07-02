use crate::{Rule, RuleDescribe, Symbol, symbols::Keyword};
use syntax::terms::{Ref, Term};

impl<T> RuleDescribe for Ref<T>
where
    T: Term,
{
    fn rule() -> Rule {
        Rule::new(
            Symbol::ctor(Keyword::Ref, None, vec![Symbol::Term]),
            "Reference",
        )
    }
}
