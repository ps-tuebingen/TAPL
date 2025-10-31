use crate::{GrammarRule, GrammarRuleDescribe, Symbol, symbols::SpecialChar};
use syntax::{language::Language, types::Mu};

impl<Lang> GrammarRuleDescribe for Mu<Lang>
where
    Lang: Language,
{
    fn rule() -> GrammarRule {
        GrammarRule::new(
            vec![
                SpecialChar::Mu.into(),
                Symbol::Variable,
                SpecialChar::Dot.into(),
                Symbol::Type,
            ]
            .into(),
            "Mu Type",
        )
    }
}
