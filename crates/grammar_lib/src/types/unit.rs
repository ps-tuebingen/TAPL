use crate::{Rule, RuleDescribe, symbols::Keyword};
use syntax::types::{Type, Unit};

impl<Ty> RuleDescribe for Unit<Ty>
where
    Ty: Type,
{
    fn rule() -> Rule {
        Rule::new(Keyword::UnitTy.into(), "Unit Type")
    }
}
