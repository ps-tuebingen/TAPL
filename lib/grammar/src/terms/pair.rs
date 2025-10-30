use crate::{GrammarRuleDescribe, Rule, Symbol, symbols::SpecialChar};
use syntax::{language::Language, terms::Pair};

impl<Lang> GrammarRuleDescribe for Pair<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(
            vec![
                SpecialChar::BrackO.into(),
                Symbol::many(Symbol::Term),
                SpecialChar::BrackC.into(),
            ]
            .into(),
            "Pair",
        )
    }
}
