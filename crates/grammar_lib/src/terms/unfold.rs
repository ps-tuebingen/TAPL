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
        Rule::new(Symbol::call_annot("unfold", 1, Symbol::Term), "Unfold")
    }
}
