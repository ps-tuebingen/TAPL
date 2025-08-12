use crate::{Rule, RuleDescribe, Symbol, symbols::Keyword};
use syntax::{
    types::Type,
    values::{Fold, Value},
};

impl<V, Ty> RuleDescribe for Fold<V, Ty>
where
    V: Value,
    Ty: Type,
{
    fn rule() -> Rule {
        Rule::new(
            Symbol::ctor(Keyword::Fold, Some(Symbol::Type), vec![Symbol::Value]),
            "Fold",
        )
    }
}
