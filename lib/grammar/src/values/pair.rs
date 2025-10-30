use crate::{GrammarRuleDescribe, Rule, Symbol, symbols::SpecialChar};
use syntax::{language::Language, values::Pair};

impl<Lang> GrammarRuleDescribe for Pair<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(
            vec![
                SpecialChar::BrackO.into(),
                Symbol::Many(Box::new(Symbol::Value)),
                SpecialChar::BrackC.into(),
            ]
            .into(),
            "Pair",
        )
    }
}
