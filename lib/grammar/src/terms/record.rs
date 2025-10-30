use crate::{GrammarRuleDescribe, Rule, Symbol, symbols::SpecialChar};
use syntax::{language::Language, terms::Record};

impl<Lang> GrammarRuleDescribe for Record<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(
            vec![
                SpecialChar::BrackO.into(),
                Symbol::Many(Box::new(
                    vec![Symbol::Label, SpecialChar::Equals.into(), Symbol::Term].into(),
                )),
                SpecialChar::BrackC.into(),
            ]
            .into(),
            "Record",
        )
    }
}
