use crate::{Rule, RuleDescribe, Symbol};
use syntax::terms::{Term, TyLambda};

impl<T> RuleDescribe for TyLambda<T>
where
    T: Term,
{
    fn rule() -> Rule {
        Rule::new(Symbol::lam(Symbol::Kind, Symbol::Term), "Type Abstraction")
    }
}
