use crate::{
    GrammarRuleDescribe, Rule, Symbol,
    symbols::{Keyword, SpecialChar},
};
use syntax::{language::Language, terms::If};

impl<Lang> GrammarRuleDescribe for If<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(
            vec![
                Keyword::If.into(),
                SpecialChar::BrackO.into(),
                Symbol::Term,
                SpecialChar::BrackC.into(),
                Keyword::Else.into(),
                SpecialChar::BrackO.into(),
                Symbol::Term,
                SpecialChar::BrackC.into(),
            ]
            .into(),
            "If",
        )
    }
}
