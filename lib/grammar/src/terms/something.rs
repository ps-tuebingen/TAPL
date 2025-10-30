use crate::{
    GrammarRuleDescribe, Rule, Symbol,
    symbols::{Keyword, SpecialChar},
};
use syntax::{language::Language, terms::Something};

impl<Lang> GrammarRuleDescribe for Something<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(
            vec![
                Keyword::Something.into(),
                SpecialChar::ParenO.into(),
                Symbol::Term,
                SpecialChar::ParenC.into(),
            ]
            .into(),
            "Something",
        )
    }
}
