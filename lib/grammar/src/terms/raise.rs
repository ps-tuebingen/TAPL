use crate::{Rule, RuleDescribe, Symbol, symbols::Keyword};
use syntax::{
    terms::{Raise, Term},
    types::Type,
};

impl<T, Ty> RuleDescribe for Raise<T, Ty>
where
    T: Term,
    Ty: Type,
{
    fn rule() -> Rule {
        Rule::new(
            Symbol::ctor(Keyword::Raise, Some(Symbol::Term), vec![Symbol::Term]),
            "Raise",
        )
    }
}
