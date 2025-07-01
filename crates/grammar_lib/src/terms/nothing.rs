use crate::{Rule, RuleDescribe, Symbol};
use syntax::{
    terms::{Nothing, Term},
    types::Type,
};

impl<T, Ty> RuleDescribe for Nothing<T, Ty>
where
    T: Term,
    Ty: Type,
{
    fn rule() -> Rule {
        Rule::new(Symbol::call_annot("Nothing", 0, Symbol::Term), "Nothing")
    }
}
