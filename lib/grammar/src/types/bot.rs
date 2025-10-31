use crate::{GrammarRule, GrammarRuleDescribe, symbols::SpecialChar};
use syntax::{language::Language, types::Bot};

impl<Lang> GrammarRuleDescribe for Bot<Lang>
where
    Lang: Language,
{
    fn rule() -> GrammarRule {
        GrammarRule::new(SpecialChar::Bot.into(), "Bottom Type")
    }
}
