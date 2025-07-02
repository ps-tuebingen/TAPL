use crate::{Rule, RuleDescribe, Symbol};
use syntax::types::{Optional, Type};

impl<Ty> RuleDescribe for Optional<Ty>
where
    Ty: Type,
{
    fn rule() -> Rule {
        Rule::new(
            Symbol::ctor("Optional", Some(Symbol::Type), vec![]),
            "Option Type",
        )
    }
}
