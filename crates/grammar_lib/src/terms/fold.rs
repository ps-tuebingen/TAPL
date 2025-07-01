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
        Rule::new(Symbol::call_annot("fold", 1, Symbol::Term), "Fold")
    }
}
