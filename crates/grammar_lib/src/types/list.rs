use crate::{Rule, RuleDescribe, Symbol};
use syntax::types::{List, Type};

impl<Ty> RuleDescribe for List<Ty>
where
    Ty: Type,
{
    fn rule() -> Rule {
        Rule::new(Symbol::call("List", 1, Symbol::Type), "List Type")
    }
}
