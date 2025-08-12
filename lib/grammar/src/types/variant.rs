use crate::{Rule, RuleDescribe, Symbol};
use syntax::types::{Type, Variant};

impl<Ty> RuleDescribe for Variant<Ty>
where
    Ty: Type,
{
    fn rule() -> Rule {
        Rule::new(Symbol::variant(Symbol::Type), "Variant Type")
    }
}
