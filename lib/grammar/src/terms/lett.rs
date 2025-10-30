use crate::{
    GrammarRuleDescribe, Rule, Symbol,
    symbols::{Keyword, SpecialChar},
};
use syntax::{language::Language, terms::Let};

impl<Lang> GrammarRuleDescribe for Let<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(
            vec![
                SpecialChar::ParenO.into(),
                Symbol::Variable,
                SpecialChar::Equals.into(),
                Symbol::Term,
                SpecialChar::ParenC.into(),
                Keyword::In.into(),
                Symbol::Term,
            ]
            .into(),
            "Let Binding",
        )
    }
}
