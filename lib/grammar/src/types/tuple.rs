use crate::{Rule, RuleDescribe, Symbol};
use syntax::types::{Tuple, Type};

impl<Ty> RuleDescribe for Tuple<Ty>
where
    Ty: Type,
{
    fn rule() -> Rule {
        Rule::new(Symbol::tuple(Symbol::Type), "Tuple Type")
    }
}
