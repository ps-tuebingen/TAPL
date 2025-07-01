use crate::{Rule, RuleDescribe, Symbol};
use syntax::terms::{Term, UntypedLambda};

impl<T> RuleDescribe for UntypedLambda<T>
where
    T: Term,
{
    fn rule() -> Rule {
        Rule::new(
            Symbol::lam(Symbol::Empty, Symbol::Term),
            "Lambda Abstraction",
        )
    }
}
