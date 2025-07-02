use crate::{Rule, RuleDescribe, Symbol};
use syntax::types::{Source, Type};

impl<Ty> RuleDescribe for Source<Ty>
where
    Ty: Type,
{
    fn rule() -> Rule {
        Rule::new(
            Symbol::ctor("Source", Some(Symbol::Type), vec![]),
            "Source Type",
        )
    }
}
