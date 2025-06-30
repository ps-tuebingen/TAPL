use crate::{Rule, RuleDescribe, Symbol};
use syntax::{terms::Term, values::UntypedLambda};

impl<T> RuleDescribe for UntypedLambda<T>
where
    T: Term,
{
    fn rule() -> Rule {
        Rule::new(Symbol::lam_untyped(), "Lambda Abstraction")
    }
}
