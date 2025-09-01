use crate::{Rule, RuleDescribe, symbols::SpecialChar};
use syntax::{language::Language, types::Top};

impl<Lang> RuleDescribe for Top<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(SpecialChar::Top.into(), "Top Type")
    }
}
