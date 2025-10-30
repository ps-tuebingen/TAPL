use crate::{
    GrammarRuleDescribe, Rule, Symbol,
    symbols::{Keyword, SpecialChar},
};
use syntax::{language::Language, terms::SomeCase};

impl<Lang> GrammarRuleDescribe for SomeCase<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(
            vec![
                Keyword::Case.into(),
                Symbol::Term,
                Keyword::Of.into(),
                Keyword::Nothing.into(),
                SpecialChar::DoubleArrow.into(),
                Symbol::Term,
                SpecialChar::Pipe.into(),
                Keyword::Something.into(),
                Symbol::paren(vec![Symbol::Variable]),
                SpecialChar::DoubleArrow.into(),
                Symbol::Term,
            ]
            .into(),
            "Option Case",
        )
    }
}
