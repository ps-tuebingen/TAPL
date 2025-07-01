use crate::{Rule, RuleDescribe, Symbol};
use syntax::{
    terms::{LambdaSub, Term},
    types::Type,
};

impl<T, Ty> RuleDescribe for LambdaSub<T, Ty>
where
    T: Term,
    Ty: Type,
{
    fn rule() -> Rule {
        Rule::new(Symbol::lam(Symbol::Type, Symbol::Term), "Lambda Sub")
    }
}
