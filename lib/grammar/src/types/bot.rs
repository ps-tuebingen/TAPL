use crate::{Rule, RuleDescribe, symbols::SpecialChar};
use syntax::types::{Bot, Type};

impl<Ty> RuleDescribe for Bot<Ty>
where
    Ty: Type,
{
    fn rule() -> Rule {
        Rule::new(SpecialChar::Bot.into(), "Bottom Type")
    }
}
