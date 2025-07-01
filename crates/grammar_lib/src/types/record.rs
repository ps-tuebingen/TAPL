use crate::{Rule, RuleDescribe, Symbol};
use syntax::types::{Record, Type};

impl<Ty> RuleDescribe for Record<Ty>
where
    Ty: Type,
{
    fn rule() -> Rule {
        Rule::new(Symbol::record(Symbol::Type), "Record Type")
    }
}
