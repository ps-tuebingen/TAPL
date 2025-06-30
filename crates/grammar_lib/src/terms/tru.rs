use crate::{Rule, RuleDescribe, Symbol};
use syntax::terms::{Term, True};

impl<T> RuleDescribe for True<T>
where
    T: Term,
{
    fn rule() -> Rule {
        Rule::new(Symbol::term("true"), "True")
    }
}
