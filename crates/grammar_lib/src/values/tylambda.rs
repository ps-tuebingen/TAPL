use crate::{Rule, RuleDescribe, Symbol};
use syntax::{terms::Term, values::TyLambda};

impl<T> RuleDescribe for TyLambda<T>
where
    T: Term,
{
    fn rule() -> Rule {
        Rule::new(Symbol::lam(Symbol::Kind, Symbol::Value), "Type Abstraction")
    }
}
