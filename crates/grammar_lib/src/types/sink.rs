use crate::{Rule, RuleDescribe, Symbol};
use syntax::types::{Sink, Type};

impl<Ty> RuleDescribe for Sink<Ty>
where
    Ty: Type,
{
    fn rule() -> Rule {
        Rule::new(
            Symbol::ctor("Sink", Some(Symbol::Type), vec![]),
            "Sink Type",
        )
    }
}
