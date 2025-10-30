use crate::{
    GrammarRuleDescribe, Rule, Symbol,
    symbols::{Keyword, SpecialChar},
};
use syntax::{language::Language, terms::Unpack};

impl<Lang> GrammarRuleDescribe for Unpack<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(
            vec![
                SpecialChar::BrackO.into(),
                Symbol::Typevariable,
                SpecialChar::Comma.into(),
                Symbol::Variable,
                SpecialChar::BrackC.into(),
                SpecialChar::Equals.into(),
                Symbol::Term,
                Keyword::In.into(),
                Symbol::Term,
            ]
            .into(),
            "Unpack",
        )
    }
}
