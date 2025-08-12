use crate::{Rule, RuleDescribe, symbols::SpecialChar};
use syntax::types::Bot;

impl RuleDescribe for Bot {
    fn rule() -> Rule {
        Rule::new(SpecialChar::Bot.into(), "Bottom Type")
    }
}
