use crate::{Rule, RuleDescribe, Symbol};
use syntax::types::{Type, TypeVariable};

impl<Ty> RuleDescribe for TypeVariable<Ty>
where
    Ty: Type,
{
    fn rule() -> Rule {
        Rule::new(Symbol::Variable, "Type Variable")
    }
}
