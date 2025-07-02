use crate::{Rule, RuleDescribe, Symbol};
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
            Symbol::ctor("unfold", Some(Symbol::Type), vec![Symbol::Term]),
            "Unfold",
        )
    }
}
