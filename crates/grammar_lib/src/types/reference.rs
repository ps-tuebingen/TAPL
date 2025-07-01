use crate::{Rule, RuleDescribe, Symbol};
use syntax::types::{Reference, Type};

impl<Ty> RuleDescribe for Reference<Ty>
where
    Ty: Type,
{
    fn rule() -> Rule {
        Rule::new(Symbol::call("Reference", 1, Symbol::Type), "Reference Type")
    }
}
