use crate::{GrammarRuleDescribe, Rule, Symbol, symbols::SpecialChar};
use syntax::{language::Language, types::ForallBounded};

impl<Lang> GrammarRuleDescribe for ForallBounded<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(
            vec![
                SpecialChar::Forall.into(),
                Symbol::Typevariable,
                SpecialChar::LessColon.into(),
                Symbol::Type,
                SpecialChar::Dot.into(),
                Symbol::Type,
            ]
            .into(),
            "Universal Type",
        )
    }
}
