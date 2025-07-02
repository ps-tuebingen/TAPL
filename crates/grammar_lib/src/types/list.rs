use crate::{Rule, RuleDescribe, Symbol};
use syntax::types::{List, Type};

impl<Ty> RuleDescribe for List<Ty>
where
    Ty: Type,
{
    fn rule() -> Rule {
        Rule::new(
            Symbol::ctor("List", Some(Symbol::Type), vec![]),
            "List Type",
        )
    }
}
