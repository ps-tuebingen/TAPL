use crate::{
    GrammarRuleDescribe, Rule, Symbol,
    symbols::{Keyword, SpecialChar},
};
use syntax::{language::Language, values::Exception};

impl<Lang> GrammarRuleDescribe for Exception<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(
            vec![
                Keyword::Err.into(),
                SpecialChar::SqBrackO.into(),
                Symbol::Type,
                SpecialChar::SqBrackC.into(),
                SpecialChar::ParenO.into(),
                Symbol::Value,
                SpecialChar::ParenC.into(),
            ]
            .into(),
            "Exception",
        )
    }
}
