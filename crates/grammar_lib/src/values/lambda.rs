use crate::{Rule, RuleDescribe, Symbol};
use syntax::{terms::Term, types::Type, values::Lambda};

impl<T, Ty> RuleDescribe for Lambda<T, Ty>
where
    T: Term,
    Ty: Type,
{
    fn rule() -> Rule {
        Rule::new(
            Symbol::lam(Symbol::ty_annot(Symbol::Variable), Symbol::Term),
            "Lambda Abstraction",
        )
    }
}
