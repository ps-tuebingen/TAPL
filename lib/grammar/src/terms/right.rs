use crate::{
    GrammarRuleDescribe, Rule, Symbol,
    symbols::{Keyword, SpecialChar},
};
use syntax::{language::Language, terms::Right};

impl<Lang> GrammarRuleDescribe for Right<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(
            vec![
                Keyword::Right.into(),
                SpecialChar::SqBrackO.into(),
                Symbol::Type,
                SpecialChar::SqBrackC.into(),
                SpecialChar::ParenO.into(),
                Symbol::Term,
                SpecialChar::ParenC.into(),
            ]
            .into(),
            "Right Injection",
        )
    }
}
