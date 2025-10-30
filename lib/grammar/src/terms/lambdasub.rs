use crate::{GrammarRuleDescribe, Rule, Symbol, symbols::SpecialChar};
use syntax::{language::Language, terms::LambdaSub};

impl<Lang> GrammarRuleDescribe for LambdaSub<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(
            vec![
                SpecialChar::Lambda.into(),
                Symbol::Variable,
                SpecialChar::LessColon.into(),
                Symbol::Type,
                SpecialChar::Dot.into(),
                Symbol::Term,
            ]
            .into(),
            "Lambda Sub",
        )
    }
}
