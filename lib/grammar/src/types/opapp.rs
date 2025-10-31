use crate::{GrammarRule, GrammarRuleDescribe, Symbol, symbols::SpecialChar};
use syntax::{language::Language, types::OpApp};

impl<Lang> GrammarRuleDescribe for OpApp<Lang>
where
    Lang: Language,
{
    fn rule() -> GrammarRule {
        GrammarRule::new(
            vec![Symbol::Type, SpecialChar::Space.into(), Symbol::Type].into(),
            "Operator Application",
        )
    }
}
