use crate::{Rule, RuleDescribe, Symbol, symbols::Keyword};
use syntax::{
    terms::{Tail, Term},
    types::Type,
};

impl<T, Ty> RuleDescribe for Tail<T, Ty>
where
    T: Term,
    Ty: Type,
{
    fn rule() -> Rule {
        Rule::new(
            Symbol::ctor(Keyword::Tail, Some(Symbol::Type), vec![Symbol::Term]),
            "List Tail",
        )
    }
}
