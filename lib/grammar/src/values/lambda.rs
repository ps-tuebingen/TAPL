use crate::{GrammarRuleDescribe, Rule, Symbol, symbols::SpecialChar};
use syntax::{language::Language, values::Lambda};

impl<Lang> GrammarRuleDescribe for Lambda<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(
            vec![
                SpecialChar::Lambda.into(),
                Symbol::Variable,
                SpecialChar::Colon.into(),
                Symbol::Type,
                SpecialChar::Dot.into(),
                Symbol::Term,
            ]
            .into(),
            "Lambda Abstraction",
        )
    }
}
