use crate::{
    GrammarRuleDescribe, Rule, Symbol,
    symbols::{Keyword, SpecialChar},
};
use syntax::{language::Language, terms::Ref};

impl<Lang> GrammarRuleDescribe for Ref<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(
            vec![
                Keyword::Ref.into(),
                SpecialChar::ParenO.into(),
                Symbol::Term,
                SpecialChar::ParenO.into(),
            ]
            .into(),
            "Reference",
        )
    }
}
