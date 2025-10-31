use crate::{GrammarRule, GrammarRuleDescribe, symbols::SpecialChar};
use syntax::{language::Language, types::Top};

impl<Lang> GrammarRuleDescribe for Top<Lang>
where
    Lang: Language,
{
    fn rule() -> GrammarRule {
        GrammarRule::new(SpecialChar::Top.into(), "Top Type")
    }
}
