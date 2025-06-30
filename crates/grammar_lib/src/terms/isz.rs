use crate::{Rule, RuleDescribe, Symbol};
use syntax::terms::{IsZero, Term};

impl<T> RuleDescribe for IsZero<T>
where
    T: Term,
{
    fn rule() -> Rule {
        Rule::new(Symbol::call_annot("iszero", 1), "IsZero")
    }
}
