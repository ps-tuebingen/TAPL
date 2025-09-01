use crate::{Rule, RuleDescribe, symbols::SpecialChar};
use syntax::{language::Language, values::Num};

impl<Lang> RuleDescribe for Num<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(SpecialChar::Number.into(), "Number")
    }
}
