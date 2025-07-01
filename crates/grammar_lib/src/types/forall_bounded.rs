use crate::{Rule, RuleDescribe, Symbol};
use syntax::types::{ForallBounded, Type};

impl<Ty> RuleDescribe for ForallBounded<Ty>
where
    Ty: Type,
{
    fn rule() -> Rule {
        Rule::new(Symbol::Forall { bounded: true }, "Universal Type")
    }
}
