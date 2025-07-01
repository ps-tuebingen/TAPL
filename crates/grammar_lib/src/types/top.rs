use crate::{Rule, RuleDescribe, Symbol};
use syntax::types::{Top, Type};

impl<Ty> RuleDescribe for Top<Ty>
where
    Ty: Type,
{
    fn rule() -> Rule {
        Rule::new(Symbol::term("Top"), "Top Type")
    }
}
