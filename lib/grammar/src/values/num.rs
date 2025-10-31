use crate::{GrammarRule, GrammarRuleDescribe, symbols::SpecialChar};
use syntax::{language::Language, values::Num};

impl<Lang> GrammarRuleDescribe for Num<Lang>
where
    Lang: Language,
{
    fn rule() -> GrammarRule {
        GrammarRule::new(SpecialChar::Number.into(), "Number")
    }
}
