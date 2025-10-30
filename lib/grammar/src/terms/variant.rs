use crate::{GrammarRuleDescribe, Rule, Symbol, symbols::SpecialChar};
use syntax::{language::Language, terms::Variant};

impl<Lang> GrammarRuleDescribe for Variant<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(
            vec![
                SpecialChar::AngBrackO.into(),
                Symbol::Many(Box::new(
                    vec![Symbol::Label, SpecialChar::Equals.into(), Symbol::Term].into(),
                )),
                SpecialChar::AngBrackC.into(),
            ]
            .into(),
            "Variant",
        )
    }
}
