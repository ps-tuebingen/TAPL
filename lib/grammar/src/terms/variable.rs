use crate::{Rule, RuleDescribe, Symbol};
use syntax::terms::{Term, Variable};

impl<T> RuleDescribe for Variable<T>
where
    T: Term,
{
    fn rule() -> Rule {
        Rule::new(Symbol::Variable, "Variable")
    }
}
