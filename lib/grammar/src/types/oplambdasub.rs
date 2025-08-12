use crate::{Rule, RuleDescribe, Symbol};
use syntax::types::{OpLambdaSub, Type};

impl<Ty> RuleDescribe for OpLambdaSub<Ty>
where
    Ty: Type,
{
    fn rule() -> Rule {
        Rule::new(
            Symbol::lam(Symbol::Type, Symbol::Type),
            "Operator Abstraction",
        )
    }
}
