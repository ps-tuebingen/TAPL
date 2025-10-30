use crate::{
    GrammarRuleDescribe, Rule, Symbol,
    symbols::{Keyword, SpecialChar},
};
use syntax::{language::Language, values::Fold};

impl<Lang> GrammarRuleDescribe for Fold<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(
            vec![
                Keyword::Fold.into(),
                SpecialChar::SqBrackO.into(),
                Symbol::Type,
                SpecialChar::SqBrackC.into(),
                SpecialChar::SqBrackC.into(),
                SpecialChar::ParenO.into(),
                Symbol::Value,
                SpecialChar::ParenC.into(),
            ]
            .into(),
            "Fold",
        )
    }
}
