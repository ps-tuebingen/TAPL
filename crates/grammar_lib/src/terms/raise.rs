use crate::{Rule, RuleDescribe, Symbol};
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
        Rule::new(Symbol::call_annot("raise", 1, Symbol::Term), "Raise")
    }
}
