use crate::{Rule, RuleDescribe, Symbol};
use syntax::{
    terms::{Right, Term},
    types::Type,
};

impl<T, Ty> RuleDescribe for Right<T, Ty>
where
    T: Term,
    Ty: Type,
{
    fn rule() -> Rule {
        Rule::new(
            Symbol::call_annot("inr", 1, Symbol::Term),
            "Right Injection",
        )
    }
}
