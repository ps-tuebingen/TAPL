use crate::{
    GrammarRuleDescribe, Rule, Symbol,
    symbols::{Keyword, SpecialChar},
};
use syntax::{language::Language, values::Something};

impl<Lang> GrammarRuleDescribe for Something<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(
            vec![
                Keyword::Something.into(),
                SpecialChar::ParenO.into(),
                Symbol::Value,
                SpecialChar::ParenC.into(),
            ]
            .into(),
            "Something",
        )
    }
}
