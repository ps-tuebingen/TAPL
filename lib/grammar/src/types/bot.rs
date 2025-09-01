use crate::{Rule, RuleDescribe, symbols::SpecialChar};
use syntax::{language::Language, types::Bot};

impl<Lang> RuleDescribe for Bot<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(SpecialChar::Bot.into(), "Bottom Type")
    }
}
