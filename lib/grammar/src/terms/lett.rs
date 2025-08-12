use crate::{Rule, RuleDescribe, Symbol};
use syntax::terms::{Let, Term};

impl<T> RuleDescribe for Let<T>
where
    T: Term,
{
    fn rule() -> Rule {
        Rule::new(Symbol::lett(), "Let Binding")
    }
}
