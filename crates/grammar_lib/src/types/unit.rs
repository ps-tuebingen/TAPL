use crate::{Rule, RuleDescribe, Symbol};
use syntax::types::{Type, Unit};

impl<Ty> RuleDescribe for Unit<Ty>
where
    Ty: Type,
{
    fn rule() -> Rule {
        Rule::new(Symbol::term("Unit"), "Unit Type")
    }
}
