use crate::{Rule, RuleDescribe, Symbol};
use syntax::{
    terms::{Fold, Term},
    types::Type,
};

impl<T, Ty> RuleDescribe for Fold<T, Ty>
where
    T: Term,
    Ty: Type,
{
    fn rule() -> Rule {
        Rule::new(
            Symbol::ctor("fold", Some(Symbol::Type), vec![Symbol::Term]),
            "Fold",
        )
    }
}
