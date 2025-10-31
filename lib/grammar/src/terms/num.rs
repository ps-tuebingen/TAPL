use crate::{GrammarRule, GrammarRuleDescribe, symbols::SpecialChar};
use syntax::{language::Language, terms::Num};

impl<Lang> GrammarRuleDescribe for Num<Lang>
where
    Lang: Language,
{
    fn rule() -> GrammarRule {
        GrammarRule::new(SpecialChar::Number.into(), "Number")
    }
}
