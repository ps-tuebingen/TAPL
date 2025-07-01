use crate::{Rule, RuleDescribe, Symbol};
use syntax::terms::{RecordProj, Term};

impl<T> RuleDescribe for RecordProj<T>
where
    T: Term,
{
    fn rule() -> Rule {
        Rule::new(Symbol::dot("label"), "Record Projection")
    }
}
