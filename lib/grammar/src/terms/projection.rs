use crate::{GrammarRuleDescribe, Rule, Symbol, symbols::SpecialChar};
use syntax::{language::Language, terms::Projection};

impl<Lang> GrammarRuleDescribe for Projection<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(
            vec![
                Symbol::Term,
                SpecialChar::Dot.into(),
                SpecialChar::Number.into(),
            ]
            .into(),
            "Projection",
        )
    }
}
