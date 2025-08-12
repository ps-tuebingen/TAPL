use crate::{Rule, RuleDescribe, Symbol};
use syntax::types::{OpLambda, Type};

impl<Ty> RuleDescribe for OpLambda<Ty>
where
    Ty: Type,
{
    fn rule() -> Rule {
        Rule::new(
            Symbol::lam(Symbol::Kind, Symbol::Type),
            "Operator Abstraction",
        )
    }
}
