use crate::{Rule, RuleDescribe, Symbol, symbols::Keyword};
use syntax::types::{List, Type};

impl<Ty> RuleDescribe for List<Ty>
where
    Ty: Type,
{
    fn rule() -> Rule {
        Rule::new(
            Symbol::ctor(Keyword::List, Some(Symbol::Type), vec![]),
            "List Type",
        )
    }
}
