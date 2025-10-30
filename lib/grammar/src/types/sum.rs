use crate::{GrammarRuleDescribe, Rule, Symbol, symbols::SpecialChar};
use syntax::{language::Language, types::Sum};

impl<Lang> GrammarRuleDescribe for Sum<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(
            vec![Symbol::Type, SpecialChar::Plus.into(), Symbol::Type].into(),
            "Sum Type",
        )
    }
}
