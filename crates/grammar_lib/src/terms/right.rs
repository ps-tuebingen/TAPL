use crate::{Rule, RuleDescribe, Symbol, symbols::Keyword};
use syntax::{
    terms::{Right, Term},
    types::Type,
};

impl<T, Ty> RuleDescribe for Right<T, Ty>
where
    T: Term,
    Ty: Type,
{
    fn rule() -> Rule {
        Rule::new(
            Symbol::ctor(Keyword::Right, Some(Symbol::Type), vec![Symbol::Term]),
            "Right Injection",
        )
    }
}
