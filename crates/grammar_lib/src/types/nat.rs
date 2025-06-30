use crate::{Rule, RuleDescribe, Symbol};
use syntax::types::{Nat, Type};

impl<Ty> RuleDescribe for Nat<Ty>
where
    Ty: Type,
{
    fn rule() -> Rule {
        Rule::new(Symbol::term("Nat"), "Natural Numbers")
    }
}
