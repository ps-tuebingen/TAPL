use crate::{
    GrammarRuleDescribe, Rule, Symbol,
    symbols::{Keyword, SpecialChar},
};
use syntax::{language::Language, terms::Snd};

impl<Lang> GrammarRuleDescribe for Snd<Lang>
where
    Lang: Language,
{
    fn rule() -> Rule {
        Rule::new(
            vec![Symbol::Term, SpecialChar::Dot.into(), Keyword::Snd.into()].into(),
            "Second Projection",
        )
    }
}
