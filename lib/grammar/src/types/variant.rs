use crate::{GrammarRule, GrammarRuleDescribe, Symbol, symbols::SpecialChar};
use syntax::{language::Language, types::Variant};

impl<Lang> GrammarRuleDescribe for Variant<Lang>
where
    Lang: Language,
{
    fn rule() -> GrammarRule {
        GrammarRule::new(
            Symbol::angbrack(Symbol::Many(Box::new(
                vec![Symbol::Label, SpecialChar::Equals.into(), Symbol::Type].into(),
            ))),
            "Variant Type",
        )
    }
}
