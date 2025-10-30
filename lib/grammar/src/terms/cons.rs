use crate::{
    GrammarRuleDescribe, Rule, Symbol,
    symbols::{Keyword, SpecialChar},
};
use syntax::{language::Language, terms::Cons};

impl<Lang> GrammarRuleDescribe for Cons<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(
            vec![
                Keyword::Cons.into(),
                SpecialChar::SqBrackO.into(),
                Symbol::Type,
                SpecialChar::SqBrackC.into(),
                SpecialChar::ParenO.into(),
                Symbol::Term,
                SpecialChar::Comma.into(),
                Symbol::Term,
                SpecialChar::ParenC.into(),
            ]
            .into(),
            "Cons",
        )
    }
}
