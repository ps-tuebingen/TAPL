use crate::{Rule, RuleDescribe, Symbol, symbols::Keyword};
use syntax::{
    terms::{Term, Unfold},
    types::Type,
};

impl<T, Ty> RuleDescribe for Unfold<T, Ty>
where
    T: Term,
    Ty: Type,
{
    fn rule() -> Rule {
        Rule::new(
            Symbol::ctor(Keyword::Unfold, Some(Symbol::Type), vec![Symbol::Term]),
            "Unfold",
        )
    }
}
