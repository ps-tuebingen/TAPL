use crate::{GrammarRuleDescribe, Rule, symbols::SpecialChar};
use syntax::{language::Language, types::Bot};

impl<Lang> GrammarRuleDescribe for Bot<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(SpecialChar::Bot.into(), "Bottom Type")
    }
}
