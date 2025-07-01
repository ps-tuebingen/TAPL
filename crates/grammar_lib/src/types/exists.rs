use crate::{Rule, RuleDescribe, Symbol};
use syntax::types::{Exists, Type};

impl<Ty> RuleDescribe for Exists<Ty>
where
    Ty: Type,
{
    fn rule() -> Rule {
        Rule::new(Symbol::Exists { bounded: false }, "Existential Type")
    }
}
