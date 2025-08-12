use crate::{Rule, RuleDescribe, Symbol, symbols::Keyword};
use syntax::{terms::Term, types::Type, values::Exception};

impl<T, Ty> RuleDescribe for Exception<T, Ty>
where
    T: Term,
    Ty: Type,
{
    fn rule() -> Rule {
        Rule::new(
            Symbol::ctor(Keyword::Err, Some(Symbol::Type), vec![Symbol::Value]),
            "Exception",
        )
    }
}
