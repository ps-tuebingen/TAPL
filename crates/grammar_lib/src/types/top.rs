use crate::{Rule, RuleDescribe, symbols::SpecialChar};
use syntax::types::{Top, Type};

impl<Ty> RuleDescribe for Top<Ty>
where
    Ty: Type,
{
    fn rule() -> Rule {
        Rule::new(SpecialChar::Top.into(), "Top Type")
    }
}
