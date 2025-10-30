use crate::{
    GrammarRuleDescribe, Rule, Symbol,
    symbols::{Keyword, SpecialChar},
};
use syntax::{language::Language, terms::IsNil};

impl<Lang> GrammarRuleDescribe for IsNil<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(
            vec![
                Keyword::IsNil.into(),
                SpecialChar::SqBrackO.into(),
                Symbol::Type,
                SpecialChar::SqBrackC.into(),
                SpecialChar::ParenO.into(),
                Symbol::Term,
                SpecialChar::ParenC.into(),
            ]
            .into(),
            "Is Nil",
        )
    }
}
