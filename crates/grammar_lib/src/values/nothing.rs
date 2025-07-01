use crate::{Rule, RuleDescribe, Symbol};
use syntax::{terms::Term, types::Type, values::Nothing};

impl<T, Ty> RuleDescribe for Nothing<T, Ty>
where
    T: Term,
    Ty: Type,
{
    fn rule() -> Rule {
        Rule::new(Symbol::call_annot("Nothing", 0, Symbol::Value), "Nothing")
    }
}
