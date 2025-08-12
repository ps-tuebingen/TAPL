use crate::{Rule, RuleDescribe, Symbol};
use syntax::types::{OpApp, Type};

impl<Ty> RuleDescribe for OpApp<Ty>
where
    Ty: Type,
{
    fn rule() -> Rule {
        Rule::new(
            Symbol::app(Symbol::Type, Symbol::Type),
            "Operator Application",
        )
    }
}
