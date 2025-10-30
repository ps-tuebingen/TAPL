use crate::{GrammarRuleDescribe, Rule, Symbol, symbols::SpecialChar};
use syntax::{language::Language, values::Pack};

impl<Lang> GrammarRuleDescribe for Pack<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(
            vec![
                SpecialChar::BrackO.into(),
                Symbol::Value,
                SpecialChar::Comma.into(),
                Symbol::Type,
                SpecialChar::BrackC.into(),
            ]
            .into(),
            "Package",
        )
    }
}
