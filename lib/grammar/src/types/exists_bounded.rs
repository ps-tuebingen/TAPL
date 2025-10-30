use crate::{GrammarRuleDescribe, Rule, Symbol, symbols::SpecialChar};
use syntax::{language::Language, types::ExistsBounded};

impl<Lang> GrammarRuleDescribe for ExistsBounded<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(
            vec![
                SpecialChar::Exists.into(),
                Symbol::Type,
                SpecialChar::LessColon.into(),
                Symbol::Type,
                SpecialChar::Dot.into(),
                Symbol::Type,
            ]
            .into(),
            "Existential Type",
        )
    }
}
