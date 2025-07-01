use crate::{Rule, RuleDescribe, Symbol};
use syntax::types::{ExistsBounded, Type};

impl<Ty> RuleDescribe for ExistsBounded<Ty>
where
    Ty: Type,
{
    fn rule() -> Rule {
        Rule::new(Symbol::Exists { bounded: true }, "Existential Type")
    }
}
