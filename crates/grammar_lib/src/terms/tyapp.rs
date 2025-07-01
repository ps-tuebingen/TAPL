use crate::{Rule, RuleDescribe, Symbol};
use syntax::{
    terms::{Term, TyApp},
    types::Type,
};

impl<T, Ty> RuleDescribe for TyApp<T, Ty>
where
    T: Term,
    Ty: Type,
{
    fn rule() -> Rule {
        Rule::new(Symbol::app(Symbol::Term, Symbol::Type), "Type Application")
    }
}
