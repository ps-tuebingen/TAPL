use crate::{
    GrammarRuleDescribe, Rule, Symbol,
    symbols::{Keyword, SpecialChar},
};
use syntax::{language::Language, terms::Nil};

impl<Lang> GrammarRuleDescribe for Nil<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(
            vec![
                Keyword::Nil.into(),
                SpecialChar::SqBrackO.into(),
                Symbol::Term,
                SpecialChar::SqBrackC.into(),
                SpecialChar::ParenO.into(),
                Symbol::Term,
                SpecialChar::ParenC.into(),
            ]
            .into(),
            "Empty List",
        )
    }
}
