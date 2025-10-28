use crate::{GrammarRuleDescribe, Rule, symbols::SpecialChar};
use syntax::{language::Language, types::Top};

impl<Lang> GrammarRuleDescribe for Top<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(SpecialChar::Top.into(), "Top Type")
    }
}
