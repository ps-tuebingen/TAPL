use crate::{GrammarRuleDescribe, Rule, Symbol, symbols::SpecialChar};
use syntax::{language::Language, types::Tuple};

impl<Lang> GrammarRuleDescribe for Tuple<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(
            vec![
                SpecialChar::ParenO.into(),
                Symbol::many(Symbol::Type),
                SpecialChar::ParenC.into(),
            ]
            .into(),
            "Tuple Type",
        )
    }
}
