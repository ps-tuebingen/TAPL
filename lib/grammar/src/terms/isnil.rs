use crate::{Rule, RuleDescribe, Symbol, symbols::Keyword};
use syntax::{
    terms::{IsNil, Term},
    types::Type,
};

impl<T, Ty> RuleDescribe for IsNil<T, Ty>
where
    T: Term,
    Ty: Type,
{
    fn rule() -> Rule {
        Rule::new(
            Symbol::ctor(Keyword::IsNil, Some(Symbol::Type), vec![Symbol::Term]),
            "Is Nil",
        )
    }
}
