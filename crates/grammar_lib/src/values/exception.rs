use crate::{Rule, RuleDescribe, Symbol};
use syntax::{terms::Term, types::Type, values::Exception};

impl<T, Ty> RuleDescribe for Exception<T, Ty>
where
    T: Term,
    Ty: Type,
{
    fn rule() -> Rule {
        Rule::new(Symbol::call_annot("error", 0, Symbol::Value), "Exception")
    }
}
