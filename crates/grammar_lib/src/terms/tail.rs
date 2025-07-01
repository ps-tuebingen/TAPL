use crate::{Rule, RuleDescribe, Symbol};
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
        Rule::new(Symbol::call_annot("tail", 1, Symbol::Term), "List Tail")
    }
}
