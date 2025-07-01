use crate::{Rule, RuleDescribe, Symbol};
use syntax::types::Bot;

impl RuleDescribe for Bot {
    fn rule() -> Rule {
        Rule::new(Symbol::term("Bot"), "Bottom Type")
    }
}
