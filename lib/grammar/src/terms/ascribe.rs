use crate::{Rule, RuleDescribe, Symbol};
use syntax::{
    terms::{Ascribe, Term},
    types::Type,
};

impl<T, Ty> RuleDescribe for Ascribe<T, Ty>
where
    T: Term,
    Ty: Type,
{
    fn rule() -> Rule {
        Rule::new(Symbol::ty_annot(Symbol::Term), "Ascription")
    }
}
