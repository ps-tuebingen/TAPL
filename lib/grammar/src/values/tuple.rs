use crate::{GrammarRuleDescribe, Rule, Symbol, symbols::SpecialChar};
use syntax::{language::Language, values::Tuple};

impl<Lang> GrammarRuleDescribe for Tuple<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(
            vec![
                SpecialChar::ParenO.into(),
                Symbol::Many(Box::new(Symbol::Value)),
                SpecialChar::ParenC.into(),
            ]
            .into(),
            "Tuple",
        )
    }
}
