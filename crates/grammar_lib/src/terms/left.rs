use crate::{Rule, RuleDescribe, Symbol};
use syntax::{
    terms::{Left, Term},
    types::Type,
};

impl<T, Ty> RuleDescribe for Left<T, Ty>
where
    T: Term,
    Ty: Type,
{
    fn rule() -> Rule {
        Rule::new(Symbol::call_annot("inl", 1, Symbol::Term), "Left Injection")
    }
}
