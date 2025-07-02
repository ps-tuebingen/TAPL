use crate::{Rule, RuleDescribe, Symbol, symbols::Keyword};
use syntax::{terms::Term, types::Type, values::Nil};

impl<T, Ty> RuleDescribe for Nil<T, Ty>
where
    T: Term,
    Ty: Type,
{
    fn rule() -> Rule {
        Rule::new(
            Symbol::ctor(Keyword::Nil, Some(Symbol::Type), vec![Symbol::Value]),
            "Empty List",
        )
    }
}
