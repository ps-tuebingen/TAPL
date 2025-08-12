use crate::{Rule, RuleDescribe, Symbol, symbols::Keyword};
use syntax::{terms::Term, types::Type, values::Nothing};

impl<T, Ty> RuleDescribe for Nothing<T, Ty>
where
    T: Term,
    Ty: Type,
{
    fn rule() -> Rule {
        Rule::new(
            Symbol::ctor(Keyword::Nothing, Some(Symbol::Type), vec![Symbol::Value]),
            "Nothing",
        )
    }
}
