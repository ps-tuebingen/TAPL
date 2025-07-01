use crate::{Rule, RuleDescribe, Symbol};
use syntax::terms::{Record, Term};

impl<T> RuleDescribe for Record<T>
where
    T: Term,
{
    fn rule() -> Rule {
        Rule::new(Symbol::Record, "Record")
    }
}
