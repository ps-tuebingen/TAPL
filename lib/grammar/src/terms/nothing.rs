use crate::{Rule, RuleDescribe, Symbol, symbols::Keyword};
use syntax::{
    terms::{Nothing, Term},
    types::Type,
};

impl<T, Ty> RuleDescribe for Nothing<T, Ty>
where
    T: Term,
    Ty: Type,
{
    fn rule() -> Rule {
        Rule::new(
            Symbol::ctor(Keyword::Nothing, Some(Symbol::Type), vec![Symbol::Term]),
            "Nothing",
        )
    }
}
