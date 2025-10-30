use crate::{GrammarRuleDescribe, Rule, Symbol, symbols::SpecialChar};
use syntax::{language::Language, types::Product};

impl<Lang> GrammarRuleDescribe for Product<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(
            vec![Symbol::Type, SpecialChar::Times.into(), Symbol::Type].into(),
            "Product Type",
        )
    }
}
