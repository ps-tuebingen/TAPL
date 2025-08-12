use crate::{Rule, RuleDescribe, Symbol};
use syntax::terms::{App, Term};

impl<T> RuleDescribe for App<T>
where
    T: Term,
{
    fn rule() -> Rule {
        Rule::new(Symbol::app(Symbol::Term, Symbol::Term), "Application")
    }
}
