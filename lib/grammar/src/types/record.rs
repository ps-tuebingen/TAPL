use crate::{GrammarRuleDescribe, Rule, Symbol, symbols::SpecialChar};
use syntax::{language::Language, types::Record};

impl<Lang> GrammarRuleDescribe for Record<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(
            vec![
                SpecialChar::BrackO.into(),
                Symbol::Many(Box::new(
                    vec![Symbol::Label, SpecialChar::Equals.into(), Symbol::Type].into(),
                )),
                SpecialChar::BrackC.into(),
            ]
            .into(),
            "Record Type",
        )
    }
}
