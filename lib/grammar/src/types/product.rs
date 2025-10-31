use crate::{GrammarRule, GrammarRuleDescribe, Symbol, symbols::SpecialChar};
use syntax::{language::Language, types::Product};

impl<Lang> GrammarRuleDescribe for Product<Lang>
where
    Lang: Language,
{
    fn rule() -> GrammarRule {
        GrammarRule::new(
            vec![Symbol::Type, SpecialChar::Times.into(), Symbol::Type].into(),
            "Product Type",
        )
    }
}
