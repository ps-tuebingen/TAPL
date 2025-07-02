use crate::{Rule, RuleDescribe, Symbol};
use syntax::terms::{Term, TyLambda};

impl<T> RuleDescribe for TyLambda<T>
where
    T: Term,
{
    fn rule() -> Rule {
        Rule::new(
            Symbol::lam(Symbol::kind_annot(Symbol::Variable), Symbol::Term),
            "Type Abstraction",
        )
    }
}
