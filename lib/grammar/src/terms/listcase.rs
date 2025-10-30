use crate::{
    GrammarRuleDescribe, Rule, Symbol,
    symbols::{Keyword, SpecialChar},
};
use syntax::{language::Language, terms::ListCase};

impl<Lang> GrammarRuleDescribe for ListCase<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(
            vec![
                Keyword::Case.into(),
                Symbol::Term,
                Keyword::Of.into(),
                Keyword::Nil.into(),
                SpecialChar::DoubleArrow.into(),
                Symbol::Term,
                SpecialChar::Pipe.into(),
                Keyword::Cons.into(),
                SpecialChar::ParenO.into(),
                Symbol::Variable,
                SpecialChar::Comma.into(),
                Symbol::Variable,
                SpecialChar::ParenC.into(),
                SpecialChar::DoubleArrow.into(),
                Symbol::Term,
            ]
            .into(),
            "Cast",
        )
    }
}
