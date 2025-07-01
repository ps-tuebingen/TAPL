use crate::{Rule, RuleDescribe, Symbol};
use syntax::{
    terms::{Term, Variant},
    types::Type,
};

impl<T, Ty> RuleDescribe for Variant<T, Ty>
where
    T: Term,
    Ty: Type,
{
    fn rule() -> Rule {
        Rule::new(Symbol::variant(Symbol::Term), "Variant")
    }
}
