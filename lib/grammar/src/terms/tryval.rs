use crate::{
    GrammarRuleDescribe, Rule, Symbol,
    symbols::{Keyword, SpecialChar},
};
use syntax::{language::Language, terms::TryWithVal};

impl<Lang> GrammarRuleDescribe for TryWithVal<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(
            vec![
                Keyword::Try.into(),
                SpecialChar::BrackO.into(),
                Symbol::Term,
                SpecialChar::BrackC.into(),
                Keyword::Catch.into(),
                SpecialChar::BrackO.into(),
                Symbol::Term,
                SpecialChar::BrackC.into(),
            ]
            .into(),
            "Try-Catch",
        )
    }
}
