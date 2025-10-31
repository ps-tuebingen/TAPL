use crate::{GrammarRule, GrammarRuleDescribe, Symbol, symbols::SpecialChar};
use syntax::{language::Language, types::Sum};

impl<Lang> GrammarRuleDescribe for Sum<Lang>
where
    Lang: Language,
{
    fn rule() -> GrammarRule {
        GrammarRule::new(
            vec![Symbol::Type, SpecialChar::Plus.into(), Symbol::Type].into(),
            "Sum Type",
        )
    }
}
