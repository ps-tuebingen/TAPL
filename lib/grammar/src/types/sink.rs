use crate::{Rule, RuleDescribe, Symbol, symbols::Keyword};
use syntax::types::{Sink, Type};

impl<Ty> RuleDescribe for Sink<Ty>
where
    Ty: Type,
{
    fn rule() -> Rule {
        Rule::new(
            Symbol::ctor(Keyword::Sink, Some(Symbol::Type), vec![]),
            "Sink Type",
        )
    }
}
