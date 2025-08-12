use crate::{Rule, RuleDescribe, symbols::Keyword};
use syntax::types::{Nat, Type};

impl<Ty> RuleDescribe for Nat<Ty>
where
    Ty: Type,
{
    fn rule() -> Rule {
        Rule::new(Keyword::Nat.into(), "Natural Numbers")
    }
}
