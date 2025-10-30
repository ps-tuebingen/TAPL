use crate::{GrammarRuleDescribe, Rule, Symbol, symbols::SpecialChar};
use syntax::{language::Language, types::Exists};

impl<Lang> GrammarRuleDescribe for Exists<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(
            vec![
                SpecialChar::Exists.into(),
                Symbol::Type,
                SpecialChar::DoubleColon.into(),
                Symbol::Kind,
                SpecialChar::Dot.into(),
                Symbol::Type,
            ]
            .into(),
            "Existential Type",
        )
    }
}
